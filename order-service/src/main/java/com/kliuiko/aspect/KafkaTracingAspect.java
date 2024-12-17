package com.kliuiko.aspect;

import io.opentelemetry.api.GlobalOpenTelemetry;
import io.opentelemetry.api.trace.Span;
import io.opentelemetry.api.trace.Tracer;
import io.opentelemetry.context.Scope;
import org.aspectj.lang.annotation.Aspect;
import org.aspectj.lang.annotation.Before;
import org.aspectj.lang.annotation.Pointcut;
import org.springframework.stereotype.Component;

@Aspect
@Component
public class KafkaTracingAspect {

    private final Tracer tracer;

    public KafkaTracingAspect() {
        // Initialize OpenTelemetry tracer
        this.tracer = GlobalOpenTelemetry.getTracer("spring-boot-kafka");
    }

    @Pointcut("execution(* org.springframework.kafka.core.KafkaTemplate.send(..))")
    public void kafkaSend() {
        // Pointcut for KafkaTemplate's send method
    }

    @Before("kafkaSend()")
    public void traceKafkaSend() {
        // Create a new span for each Kafka message send
        Span span = tracer.spanBuilder("Kafka Produce")
                .setAttribute("messaging.destination", "your-topic-name") // Optionally specify topic name
                .startSpan();

        // Attach the span to the current context (allowing it to propagate)
        try (Scope scope = span.makeCurrent()) {
            // Perform the Kafka send operation
            // Kafka send logic will proceed as usual
        } finally {
            span.end(); // Ensure the span is ended after the operation
        }
    }
}
