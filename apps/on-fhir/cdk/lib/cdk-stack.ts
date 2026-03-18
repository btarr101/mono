import * as cdk from "aws-cdk-lib";
import * as s3 from "aws-cdk-lib/aws-s3";
import * as s3deploy from "aws-cdk-lib/aws-s3-deployment";
import * as ec2 from "aws-cdk-lib/aws-ec2";
import * as iam from "aws-cdk-lib/aws-iam";
import * as cloudfront from "aws-cdk-lib/aws-cloudfront";
import * as origins from "aws-cdk-lib/aws-cloudfront-origins";
import * as logs from "aws-cdk-lib/aws-logs";

import { Construct } from "constructs";

export class CdkStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const fhirServer = this.createFhirServer();
    const deployBucket = this.createProviderAppDeployBucket();

    const fhirBehavior = {
      origin: new origins.HttpOrigin(fhirServer.instancePublicDnsName, {
        protocolPolicy: cloudfront.OriginProtocolPolicy.HTTP_ONLY,
        customHeaders: {
          Forwarded: "proto=https", // !Important! Fhir server uses this to deduce it's base URL
        },
      }),
      allowedMethods: cloudfront.AllowedMethods.ALLOW_ALL,
      cachePolicy: cloudfront.CachePolicy.CACHING_DISABLED,
      originRequestPolicy: cloudfront.OriginRequestPolicy.ALL_VIEWER,
      viewerProtocolPolicy: cloudfront.ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
    } satisfies cloudfront.BehaviorOptions;

    new cloudfront.Distribution(this, "distribution", {
      defaultRootObject: "index.html",
      defaultBehavior: {
        origin: origins.S3BucketOrigin.withOriginAccessControl(deployBucket),
        viewerProtocolPolicy: cloudfront.ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
      },
      additionalBehaviors: {
        fhir: fhirBehavior,
        "fhir/*": fhirBehavior,
      },
    });
  }

  createFhirServer() {
    const defaultVpc = ec2.Vpc.fromLookup(this, "vpc", {
      isDefault: true,
    });

    const instance = new ec2.Instance(this, "fhir-server-instance", {
      vpc: defaultVpc,
      instanceType: ec2.InstanceType.of(
        ec2.InstanceClass.T4G,
        ec2.InstanceSize.SMALL,
      ),
      machineImage: ec2.MachineImage.latestAmazonLinux2({
        cpuType: ec2.AmazonLinuxCpuType.ARM_64,
      }),
      vpcSubnets: { subnetType: ec2.SubnetType.PUBLIC },
      associatePublicIpAddress: true,
      userDataCausesReplacement: true,
    });
    instance.connections.allowFromAnyIpv4(ec2.Port.tcp(80));
    instance.role.addManagedPolicy(
      iam.ManagedPolicy.fromAwsManagedPolicyName(
        "AmazonSSMManagedInstanceCore",
      ),
    );
    instance.role.addManagedPolicy(
      iam.ManagedPolicy.fromAwsManagedPolicyName("CloudWatchLogsFullAccess"),
    );

    const logGroup = new logs.LogGroup(this, "fhir-server-log-group", {
      retention: logs.RetentionDays.ONE_WEEK,
    });
    logGroup.grantWrite(instance.role);

    instance.addUserData(
      [
        "yum update -y",
        "amazon-linux-extras install docker -y",
        "service docker start",
        "usermod -a -G docker ec2-user",
        `docker run -d -p 80:8080 \
        -e SERVER_FORWARD_HEADERS_STRATEGY=framework \
        -e SPRINGDOC_API_DOCS_SERVER_URL=/fhir \
        --log-driver=awslogs \
        --log-opt awslogs-region=${cdk.Stack.of(this).region} \
        --log-opt awslogs-group=${logGroup.logGroupName} \
        --log-opt awslogs-stream=$(hostname) \
        --restart unless-stopped \
        hapiproject/hapi:latest`,
      ].join("\n"),
    );

    return instance;
  }

  createProviderAppDeployBucket() {
    const bucket = new s3.Bucket(this, "bucket", {
      blockPublicAccess: s3.BlockPublicAccess.BLOCK_ALL,
      objectOwnership: s3.ObjectOwnership.BUCKET_OWNER_ENFORCED,
    });
    new s3deploy.BucketDeployment(this, "deployment", {
      sources: [s3deploy.Source.asset("../provider-app/dist")],
      destinationBucket: bucket,
    });

    return bucket;
  }
}
