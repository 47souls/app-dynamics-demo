# syntax=docker/dockerfile:1
FROM openjdk:18-alpine AS build
COPY javaagent.jar javaagent.jar
COPY conf conf

RUN addgroup -S spring && adduser -S spring -G spring
USER spring:spring
ARG JAR_FILE=target/*.jar
COPY ${JAR_FILE} app.jar
ENTRYPOINT ["java","-jar","app.jar", "-javaagent:javaagent.jar"]
