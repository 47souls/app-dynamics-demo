# syntax=docker/dockerfile:1
FROM amazoncorretto:17-alpine AS build
ADD app-dynamics-agent/ /app-dynamics-agent/
ARG JAR_FILE=target/*.jar
COPY ${JAR_FILE} app.jar
ENTRYPOINT ["java","-javaagent:app-dynamics-agent/javaagent.jar","-jar","app.jar"]
