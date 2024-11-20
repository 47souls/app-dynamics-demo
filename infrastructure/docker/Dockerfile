# syntax=docker/dockerfile:1

################################################################################
# Create a stage for building the application.
FROM openjdk:17-jdk-slim AS build
RUN addgroup -S spring && adduser -S spring -G spring
USER spring:spring
ARG JAR_FILE=target/*.jar
COPY ${JAR_FILE} app.jar
ENTRYPOINT ["java","-jar","/app.jar"]
