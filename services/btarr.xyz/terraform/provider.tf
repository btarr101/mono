terraform {
  backend "s3" {
    bucket       = "btarr-terraform-backend-bucket"
    region       = "us-west-1"
    key          = "btarr.xyz.tfstate"
    use_lockfile = true
  }

  required_providers {
    digitalocean = {
      source  = "digitalocean/digitalocean"
      version = "~> 2.0"
    }
  }
}

data "digitalocean_ssh_key" "ansible" {
  name = "ansible"
}
