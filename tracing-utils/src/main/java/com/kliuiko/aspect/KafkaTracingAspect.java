package com.kliuiko.aspect;

import io.opentelemetry.api.trace.Tracer;
import org.aspectj.lang.ProceedingJoinPoint;
import org.aspectj.lang.annotation.Around;
import org.aspectj.lang.annotation.Aspect;
import org.aspectj.lang.reflect.MethodSignature;
import org.springframework.kafka.annotation.KafkaListener;
import org.springframework.stereotype.Component;

import java.lang.reflect.Method;
import java.util.Collections;
import java.util.HashMap;
import java.util.Map;

@Aspect
@Component
public class KafkaTracingAspect extends TracingAspect {

    public KafkaTracingAspect(Tracer tracer) {
        super(tracer);
    }

    @Around("execution(* org.springframework.kafka.core.KafkaTemplate.send(..)) && within(@com.kliuiko.aspect.EnableTracing *)")
    public Object aroundKafkaProduce(ProceedingJoinPoint joinPoint) {
        return decorateWithTracing(joinPoint, "Kafka message send", "kafka");
    }

    @Around("@annotation(org.springframework.kafka.annotation.KafkaListener)) && within(@com.kliuiko.aspect.EnableTracing *)")
    public Object aroundKafkaConsume(ProceedingJoinPoint joinPoint) {
        // Get the method being executed
        MethodSignature signature = (MethodSignature) joinPoint.getSignature();
        Method method = signature.getMethod();

        // Get the KafkaListener annotation
        KafkaListener kafkaListener = method.getAnnotation(KafkaListener.class);

        Map<String, String> attributesMap = new HashMap<>();

        if (kafkaListener != null) {
            // Retrieve annotation values
            String groupId = kafkaListener.groupId();
            String[] topics = kafkaListener.topics();
            attributesMap.put("groupId", groupId);
            attributesMap.put("topics", String.join(", ", topics));
        }
        return decorateWithTracing(joinPoint, "Kafka message receive", "kafka", attributesMap);
    }
}
