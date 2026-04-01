resource "digitalocean_droplet" "droplet" {
  image    = "debian-13-x64"
  name     = "btarr.xyz"
  region   = "sfo3"
  size     = "s-1vcpu-1gb"
  ssh_keys = [data.digitalocean_ssh_key.terraform.id]
}