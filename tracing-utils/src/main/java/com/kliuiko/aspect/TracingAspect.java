package com.kliuiko.aspect;

import io.opentelemetry.api.trace.Span;
import io.opentelemetry.api.trace.SpanBuilder;
import io.opentelemetry.api.trace.Tracer;
import io.opentelemetry.context.Scope;
import lombok.SneakyThrows;
import lombok.extern.slf4j.Slf4j;
import org.aspectj.lang.ProceedingJoinPoint;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Component;

import java.util.Collections;
import java.util.Map;

@Component
@Slf4j
public class TracingAspect {

    protected final Tracer tracer;

    @Autowired
    public TracingAspect(final Tracer tracer) {
        this.tracer = tracer;
    }

    @SneakyThrows
    protected Object decorateWithTracing(ProceedingJoinPoint joinPoint, String spanName,
                                         String operationKind, Map<String, String> attributesMap) {
        String methodName = joinPoint.getSignature().getName();

        // Create a new span for each Kafka message send
        SpanBuilder spanBuilder = tracer.spanBuilder(spanName)
                .setAttribute(operationKind + " operation", methodName);
        attributesMap.forEach(spanBuilder::setAttribute);
        Span span = spanBuilder.startSpan();

        Object result;
        // Attach the span to the current context (allowing it to propagate)
        try (final Scope scope = span.makeCurrent()) {
            // Proceed with the actual method execution
            result = joinPoint.proceed();
        } finally {
            span.end();
        }
        return result;
    }

    @SneakyThrows
    protected Object decorateWithTracing(ProceedingJoinPoint joinPoint, String spanName, String operationKind) {
        return decorateWithTracing(joinPoint, spanName, operationKind, Collections.emptyMap());
    }
}
