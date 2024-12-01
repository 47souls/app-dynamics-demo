FROM hashicorp/terraform:latest

RUN apk add --no-cache curl

RUN mkdir /app
ADD $PWD /app
ADD entrypoint.sh /app/entrypoint.sh

# terraform state directory
RUN mkdir /app/.state
VOLUME .state

WORKDIR /app

RUN chmod +x entrypoint.sh

ENTRYPOINT ["./entrypoint.sh"]