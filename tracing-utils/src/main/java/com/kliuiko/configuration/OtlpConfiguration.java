package com.kliuiko.configuration;

import io.opentelemetry.exporter.otlp.http.trace.OtlpHttpSpanExporter;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

@Configuration
public class OtlpConfiguration {

    @Value("${OTEL_EXPORTER_ENDPOINT:http://localhost:4318/v1/traces}")
    private String otlpEndpoint;

    @Bean
    public OtlpHttpSpanExporter otlpHttpSpanExporter() {
        return OtlpHttpSpanExporter.builder()
                .setEndpoint(otlpEndpoint)
                .build();
    }

}
