#!/usr/bin/env node
import * as cdk from "aws-cdk-lib";

import { TerraformBackendStack } from "../lib/terraform-backend-stack";

const app = new cdk.App();

new TerraformBackendStack(app, "TerraformBackendStack", {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "us-west-1",
  },
});
