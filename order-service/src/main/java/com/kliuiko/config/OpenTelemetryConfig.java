package com.kliuiko.config;

import io.opentelemetry.api.GlobalOpenTelemetry;
import io.opentelemetry.api.trace.Tracer;
import io.opentelemetry.exporter.jaeger.JaegerGrpcSpanExporter;
import io.opentelemetry.sdk.OpenTelemetrySdk;
import io.opentelemetry.sdk.trace.SdkTracerProvider;
import io.opentelemetry.sdk.trace.export.SimpleSpanProcessor;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

@Configuration
public class OpenTelemetryConfig {

    @Bean
    public Tracer tracer() {
        // Set up the Jaeger exporter (or any other exporter you're using)
        try (JaegerGrpcSpanExporter jaegerGrpcSpanExporter = JaegerGrpcSpanExporter.builder()
                .setEndpoint("http://localhost:14250") // Replace with your Jaeger server endpoint
                .build()) {
            // Add the Jaeger exporter to the OpenTelemetry SDK
            OpenTelemetrySdk.builder()
                    .setTracerProvider(SdkTracerProvider.builder()
                            .addSpanProcessor(SimpleSpanProcessor.create(jaegerGrpcSpanExporter))
                            .build())
                    .buildAndRegisterGlobal();

            return GlobalOpenTelemetry.getTracer("order-service");
        }
    }
}
