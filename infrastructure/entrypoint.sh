#!/bin/sh
terraform "$@"

if [ $# -eq 0 ]; then
  echo "no terraform command provided. exiting..."
  exit 1
fi

command=$1

case $1 in
    "init")
        terraform init
        ;;
    "plan")
        terraform init
        terraform plan
        ;;
    "apply")
        terraform init
        terraform plan
        terraform apply -auto-approve
        ;;
    "destroy")
        terraform init
        terraform plan
        terraform destroy -auto-approve
        ;;
    *)
        echo "invalid command. available commands: init, plan, apply, destroy."
        ;;
esac