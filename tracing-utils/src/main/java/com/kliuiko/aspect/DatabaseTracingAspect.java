package com.kliuiko.aspect;

import io.opentelemetry.api.trace.Tracer;
import lombok.extern.slf4j.Slf4j;
import org.aspectj.lang.ProceedingJoinPoint;
import org.aspectj.lang.annotation.Around;
import org.aspectj.lang.annotation.Aspect;
import org.springframework.stereotype.Component;

import java.util.Collections;

@Aspect
@Component
@Slf4j
public class DatabaseTracingAspect extends TracingAspect {

    public DatabaseTracingAspect(Tracer tracer) {
        super(tracer);
    }

    @Around("execution(* org.springframework.data.repository.Repository+.*(..)) && within(@com.kliuiko.aspect.EnableTracing *))")
    public Object aroundRepositoryMethods(ProceedingJoinPoint joinPoint) {
        return decorateWithTracing(joinPoint, "Repository", "database");
    }

    @Around("execution(* org.springframework.jdbc.core.JdbcTemplate+.*(..)) && within(@com.kliuiko.aspect.EnableTracing *)")
    public Object aroundJdbcTemplateMethods(ProceedingJoinPoint joinPoint) {
        return decorateWithTracing(joinPoint, "Jdbc template", "database");
    }
}
