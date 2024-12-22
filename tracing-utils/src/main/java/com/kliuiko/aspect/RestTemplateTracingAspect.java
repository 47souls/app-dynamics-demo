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
public class RestTemplateTracingAspect extends TracingAspect {

    public RestTemplateTracingAspect(Tracer tracer) {
        super(tracer);
    }

    @Around("execution(* org.springframework.web.client.RestTemplate+.*(..))")
    public Object aroundRestTemplateMethods(ProceedingJoinPoint joinPoint) {
        return decorateWithTracing(joinPoint, "Api call", "api");
    }
}
