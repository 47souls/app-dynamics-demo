### To run provisioning commands

```shell
# build terraform with infra folder
docker build -t terraform-local .
# run container with command
docker run --rm -v $PWD/state:/app/.state terraform-local {init/plan/apply/destroy}

# to build server (from root of the project)
# for linux
docker build . -t prediction-server:latest --platform linux/amd64
```

### To push image to registry

```shell
# copy files from my local env to vm
gcloud compute scp ~/localtest.txt ~/localtest2.txt example-instance:~/narnia

# 
gcloud auth login
cd auth

gcloud auth activate-service-account prediction-bot-account@prediction-bot-408920.iam.gserviceaccount.com --key-file=auth.json
gcloud auth configure-docker us-west1-docker.pkg.dev
# to build for mac remove platform arg
# docker build . -t prediction-bot-server --platform linux/amd64 -f server/Dockerfile

docker tag prediction-server:latest us-west1-docker.pkg.dev/prediction-bot-408920/docker-repository/prediction-bot-server:latest
docker push us-west1-docker.pkg.dev/prediction-bot-408920/docker-repository/prediction-bot-server
```

### To deploy image to vm
```shell
# here it has to be project ID no project name
gcloud compute ssh --zone "europe-west10-a" "prediction-backend-prod-server" --tunnel-through-iap --project "prediction-bot-408920"
# clone repo
git clone https://github.com/47souls/prediction-backend
```

### Backup mongo db volume 
```shell
# Export data from vm
docker run --rm --volumes-from {CONTAINER_ID} -v $(pwd):/backup ubuntu tar cvf /backup/backup.tar /data/db
# Copy backup from vm
gcloud compute scp --recurse prediction-backend-prod-server:~/backup.tar .
# Import data into local env
docker run --rm --volumes-from {CONTAINER_ID} -v $(pwd):/backup ubuntu bash -c "cd /data/db && tar xvf /backup/backup.tar --strip 1"
```

### Cleanup build cache at vm (free up space)
```shell
# free many space but next build will be slooooooow (25 mins)
docker builder prune
```

### Export/import production bets

```shell
# to export bot bets from container
mongoexport --db=prediction -u admin -p admin -c=bot_bet --jsonArray "mongodb://admin:admin@localhost:27017/prediction?authSource=admin" > bot_bet_fresh.json 

# copy file from container to vm
docker cp {CONTAINER_ID}:bot_bet.json .

# copy file from vm to local
gcloud compute scp --recurse prediction-backend-prod-server:~/bot_bet.json .

# copy file into local mongo container
docker cp bot_bet.json {CONTAINER_ID}:.

# import data into mongo
mongoimport --db=prediction -u admin -p admin -c=bot_bet --file=bot_bet_fresh.json --jsonArray "mongodb://admin:admin@localhost:27017/prediction?authSource=admin"
```
