package com.kliuiko.aspect;

import io.opentelemetry.api.trace.Span;
import io.opentelemetry.api.trace.Tracer;
import io.opentelemetry.context.Scope;
import lombok.extern.slf4j.Slf4j;
import org.aspectj.lang.ProceedingJoinPoint;
import org.aspectj.lang.annotation.Around;
import org.aspectj.lang.annotation.Aspect;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Component;

@Aspect
@Component
@Slf4j
public class DatabaseTracingAspect {

    private final Tracer tracer;

    @Autowired
    public DatabaseTracingAspect(final Tracer tracer) {
        this.tracer = tracer;
    }

    @Around("execution(* org.springframework.data.repository.Repository+.*(..))")
    public Object aroundRepositoryMethods(ProceedingJoinPoint joinPoint) throws Throwable {
        String methodName = joinPoint.getSignature().getName();
        log.info("Before executing repository method: {}", methodName);

        try {
            // Create a new span for each Kafka message send
            Span span = tracer.spanBuilder("Database operation")
                    .setAttribute("database operation", methodName)
                    .startSpan();

            Object result;
            // Attach the span to the current context (allowing it to propagate)
            try (Scope scope = span.makeCurrent()) {
                // Proceed with the actual method execution
                result = joinPoint.proceed();
                log.info("After executing repository method: {}", methodName);
            }
            finally {
                span.end();
            }
            return result;
        } catch (Throwable ex) {
            log.error("Exception in repository method: {}", methodName, ex);
            throw ex;
        }
    }

//    @Pointcut("execution(* org.springframework.kafka.core.KafkaTemplate.send(..))")
//    public void kafkaSend() {
//        // Pointcut for KafkaTemplate's send method
//    }
//
//    @Before("kafkaSend()")
//    public void traceKafkaSend() {
//        // Create a new span for each Kafka message send
//        Span span = tracer.spanBuilder("Kafka Produce")
//                .setAttribute("messaging.destination", "retrieve from aop")
//                .startSpan();
//
//        // Attach the span to the current context (allowing it to propagate)
//        try (Scope scope = span.makeCurrent()) {
//            // Perform the Kafka send operation
//            kafkaSend();
//        } finally {
//            span.end();
//        }
//    }
}
