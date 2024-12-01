### To run provisioning commands

```shell
# build terraform with infra folder
docker build -t terraform-local .
# run container with command
docker run --rm -v $PWD/state:/app/.state terraform-local {init/plan/apply/destroy}

# to build server (from root of the project)
# for linux
docker build . -t order-service:latest --platform linux/amd64
```

### To push image to registry

```shell
# copy files from my local env to vm
gcloud compute scp ~/localtest.txt ~/localtest2.txt example-instance:~/narnia

gcloud auth login
cd auth

gcloud auth activate-service-account ap-dynamics-service-user@app-dynamics-demo.iam.gserviceaccount.com --key-file=auth.json
gcloud auth configure-docker europe-west10-docker.pkg.dev

docker tag service:latest europe-west10-docker.pkg.dev/app-dynamics-demo/app-dynamics-container-registry/{service}:latest
docker push europe-west10-docker.pkg.dev/app-dynamics-demo/app-dynamics-container-registry/service:latest
```

### To deploy image to vm

```shell
# here it has to be project ID no project name
gcloud compute ssh --zone "europe-west10-a" "app-dynamics-prod-server" --tunnel-through-iap --project "app-dynamics-demo"
```

### Copy machine agent into vm and start it

```shell
gcloud compute scp machine-agent.tar.gz machine-agent.tar.gz app-dynamics-prod-server:~/
tar -xvf machine-agent.tar.gz
rm machine-agent.tar.gz
sudo apt update && sudo apt install openjdk-11-jdk -y
cd machine-agent && nohup java -jar machineagent.jar &
```

### Order-service

```shell
# create an order
curl -XPOST -H "Content-type: application/json" localhost:8080/order -d '{"orderDescription": "order description"}'
```
