package com.kliuiko.aspect;

import io.opentelemetry.api.trace.Span;
import io.opentelemetry.api.trace.Tracer;
import io.opentelemetry.context.Scope;
import org.aspectj.lang.annotation.Aspect;
import org.aspectj.lang.annotation.Before;
import org.aspectj.lang.annotation.Pointcut;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Component;

@Aspect
@Component
public class KafkaTracingAspect {

    private final Tracer tracer;

    @Autowired
    public KafkaTracingAspect(final Tracer tracer) {
        this.tracer = tracer;
    }

    @Pointcut("execution(* org.springframework.kafka.core.KafkaTemplate.send(..))")
    public void kafkaSend() {
        // Pointcut for KafkaTemplate's send method
    }

    @Before("kafkaSend()")
    public void traceKafkaSend() {
        // Create a new span for each Kafka message send
        Span span = tracer.spanBuilder("Kafka Produce")
                .setAttribute("messaging.destination", "retrieve from aop")
                .startSpan();

        // Attach the span to the current context (allowing it to propagate)
        try (Scope scope = span.makeCurrent()) {
            // Perform the Kafka send operation
            kafkaSend();
        } finally {
            span.end();
        }
    }
}
