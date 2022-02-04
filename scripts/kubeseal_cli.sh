#!/bin/bash

set -e


function render_secret() {
  if [ -z "$LITERALS" ]
  then
    echo "Missing Literals"
    exit 1
  else
    SECRET_NAME="${1}"
    kubectl create secret generic ${SECRET_NAME} --dry-run=client ${LITERALS} -o json | kubeseal -o yaml
  fi 
}


# catch first arguments with $1
case "$1" in
  render)
  # Clones a repository
  render_secret ${@:2}
  ;;
 *)
  # else
  echo "Required environment variable: LITERALS => String with the format: --from-literal=KEY=VALUE"
  echo "Usage:"
  echo "  render secret_name"
  ;;
esac