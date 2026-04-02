#cloud-config
packages:
  - python3 # for ansible

users:
  - name: ansible
    lock_passwd: true
    ssh_authorized_keys:
      - ${ansible_ssh_public_key}
    sudo: ['ALL=(ALL) NOPASSWD:ALL']
    groups: sudo
    shell: /bin/bash

ssh_pwauth: false # No password login for anyone, only SSH keys

write_files:
  - path: /etc/ssh/sshd_config.d/99-hardening.conf
    content: |
      PermitRootLogin no
      PasswordAuthentication no
      AllowUsers ansible

runcmd:
  - systemctl restart ssh
