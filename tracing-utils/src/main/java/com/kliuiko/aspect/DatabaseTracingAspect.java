package com.kliuiko.aspect;

import io.opentelemetry.api.trace.Tracer;
import org.aspectj.lang.ProceedingJoinPoint;
import org.aspectj.lang.annotation.Around;
import org.aspectj.lang.annotation.Aspect;
import org.springframework.boot.autoconfigure.condition.ConditionalOnProperty;
import org.springframework.stereotype.Component;

@Aspect
@Component
@ConditionalOnProperty(name = "otlp.tracing.enabled", havingValue = "true")
public class DatabaseTracingAspect extends TracingAspect {

    public DatabaseTracingAspect(Tracer tracer) {
        super(tracer);
    }

    @Around("execution(* org.springframework.data.repository.Repository+.*(..))")
    public Object aroundRepositoryMethods(ProceedingJoinPoint joinPoint) {
        return decorateWithTracing(joinPoint, "Repository", "database");
    }

    @Around("execution(* org.springframework.jdbc.core.JdbcTemplate+.*(..))")
    public Object aroundJdbcTemplateMethods(ProceedingJoinPoint joinPoint) {
        return decorateWithTracing(joinPoint, "Jdbc template", "database");
    }
}
