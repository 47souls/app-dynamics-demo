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
----------
Most recent todos

1. Create spring boot integration tests to assert traces and spans are being created - []
2. Investigate how app dynamics and jaeger integrate in citi - [ ]
3. Find out if possible what is jaeger UAT env - [ ]
4. Start pmm-sample with javaagent and library - [ ]
5. Start pmm sample app and populate corresponding application in app dynamics - [ ]