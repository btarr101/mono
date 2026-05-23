resource "digitalocean_record" "CNAME" {
  domain = digitalocean_domain.default.name
  type   = "CNAME"
  name   = "*"
  value  = "@"
}
