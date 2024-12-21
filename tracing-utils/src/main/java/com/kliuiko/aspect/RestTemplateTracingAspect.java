package com.kliuiko.aspect;

import io.opentelemetry.api.trace.Tracer;
import org.aspectj.lang.ProceedingJoinPoint;
import org.aspectj.lang.annotation.Around;
import org.aspectj.lang.annotation.Aspect;
import org.springframework.stereotype.Component;

import java.util.Collections;

@Aspect
@Component
public class RestTemplateTracingAspect extends TracingAspect {

    public RestTemplateTracingAspect(Tracer tracer) {
        super(tracer);
    }

    @Around("execution(* org.springframework.web.client.RestTemplate+.*(..)) && within(@com.kliuiko.aspect.EnableTracing *)")
    public Object aroundRestTemplateMethods(ProceedingJoinPoint joinPoint) {
        return decorateWithTracing(joinPoint, "Api call", "api");
    }
}
