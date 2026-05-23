resource "digitalocean_domain" "default" {
  name       = "btarr.xyz"
  ip_address = digitalocean_droplet.droplet.ipv4_address
}
