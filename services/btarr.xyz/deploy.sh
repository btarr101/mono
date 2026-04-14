#!/usr/bin/env bash
set -euo pipefail

# Util functions
command -v jq >/dev/null 2>&1 && VIEW_JSON_FILE=(jq .) || VIEW_JSON_FILE=(cat)

# Vars
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if [ "${GITHUB_ACTIONS:-}" = "true" ]; then
  NO_INPUT="-input=false"
  AUTO_APPROVE="-auto-approve"
fi

# Script
TERRAFORM_OUTPUT_FILE=$(mktemp /tmp/tf_output.XXXXXX.json)
trap 'rm -f "$TERRAFORM_OUTPUT_FILE"' EXIT
(
  echo "=====================================" 
  echo "Deploying Terraform infrastructure..."
  echo "====================================="
  echo
  cd ${SCRIPT_DIR}/terraform > /dev/null

  echo "1) Loading environment w/ direnv"
  echo "--------------------------------"
  echo
  if command -v direnv >/dev/null 2>&1; then
    eval "$(direnv export bash)"
  else
    echo -e "NOTE: direnv not found, skipping environment loading."
  fi
  echo

  echo "2) Terraform init"
  echo "-----------------"
  echo
  terraform init ${NO_INPUT:-}
  echo

  echo "3) Terraform apply"
  echo "------------------"
  echo
  terraform apply ${NO_INPUT:-} ${AUTO_APPROVE:-}
  echo

  
  echo "4) Terraform output"
  echo "-------------------"
  echo
  terraform output -json > "${TERRAFORM_OUTPUT_FILE}"
  echo "'''"
  "${VIEW_JSON_FILE[@]}" "${TERRAFORM_OUTPUT_FILE}"
  echo "'''"
  echo
)

(
  echo "=================="
  echo "Running ansible..."
  echo "=================="
  echo
  cd "${SCRIPT_DIR}/ansible"

  echo "1) Installing Ansible Galaxy requirements"
  echo "-----------------------------------------"
  echo
  ansible-galaxy collection install -r requirements.yml
  echo

  echo "2) Extracting droplet IP"
  echo "------------------------"
  echo
  DROPLET_IP="$(jq -r '.droplet_ip.value' "${TERRAFORM_OUTPUT_FILE}")"
  echo "Droplet IP: ${DROPLET_IP}"
  echo

  echo "3) Building inventory"
  echo "---------------------"
  echo

  ANSIBLE_INVENTORY_FILE="$(mktemp "${PWD}/inventory.XXXXXX")"
  trap 'rm -f "$ANSIBLE_INVENTORY_FILE"' EXIT
  printf $'[all]\n%s\n' "$DROPLET_IP" > "$ANSIBLE_INVENTORY_FILE"

  echo "'''"
  echo "$(cat "$ANSIBLE_INVENTORY_FILE")"
  echo "'''"
  echo

  echo "4) Waiting for cloud-init to complete"
  echo "--------------------------------------"
  echo
  MAX_ATTEMPTS=3
  ATTEMPT=1
  while true; do
    if ANSIBLE_HOST_KEY_CHECKING=False ansible all \
      -i "$ANSIBLE_INVENTORY_FILE" \
      -u ansible \
      -m command \
      -a "cloud-init status --wait"; then
      break
    fi

    if [ "$ATTEMPT" -ge "$MAX_ATTEMPTS" ]; then
      echo "ERROR: cloud-init did not complete after ${MAX_ATTEMPTS} attempts." >&2
      exit 1
    fi

    echo "Waiting... (attempt ${ATTEMPT}/${MAX_ATTEMPTS})"
    ATTEMPT=$((ATTEMPT + 1))
    sleep 5
  done
  echo

  echo "5) Execute main playbook"
  echo "------------------------"
  echo
  ANSIBLE_HOST_KEY_CHECKING=False ansible-playbook main.yml -i "$ANSIBLE_INVENTORY_FILE"
  echo
)
