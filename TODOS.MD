### Prometheus integration

1. Start prometheus as part of `docker-compose.yml`
2. Need a machine agent installed, looks like
Ok the steps were -
- tar -czvf machine-agent.tar.gz machine-agent
- gcloud compute scp ./machine-agent.zip ./machine-agent.zip app-dynamics-prod-server:~/
At vm
- tar -xvf machine-agent.tar.gz
- cd machine-agent
- sudo apt update && sudo apt install openjdk-11-jdk -y && java -version
- nohup java -jar machineagent.jar & (to start this process at background)
3. Use https://github.com/Appdynamics/prometheus-extension 