resource "digitalocean_droplet" "droplet" {
  image  = "debian-13-x64"
  name   = "btarr.xyz"
  region = "sfo3"
  size   = "s-1vcpu-1gb"

  user_data = templatefile("${path.module}/cloud-init.yml.tpl", {
    ansible_ssh_public_key = data.digitalocean_ssh_key.ansible.public_key
  })
}
