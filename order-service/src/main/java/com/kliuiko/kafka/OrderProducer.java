package com.kliuiko.kafka;

import com.fasterxml.jackson.databind.ObjectMapper;
import com.kliuiko.model.Order;
import io.opentelemetry.api.trace.Tracer;
import io.opentelemetry.api.trace.Span;
import io.opentelemetry.context.Scope;
import lombok.SneakyThrows;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.kafka.core.KafkaTemplate;
import org.springframework.stereotype.Service;

@Service
@Slf4j
public class OrderProducer {

    private static final String ORDER_TOPIC = "order";
    private final KafkaTemplate<String, String> kafkaTemplate;
    private final Tracer tracer;

    @Autowired
    public OrderProducer(final KafkaTemplate<String, String> kafkaTemplate,
                         final Tracer tracer) {
        this.kafkaTemplate = kafkaTemplate;
        this.tracer = tracer;
    }

    @SneakyThrows
    public void sendMessage(Order order) {
        // Start a new span to trace the Kafka send operation
        Span span = tracer.spanBuilder("order-send-span").startSpan();

        try (Scope scope = span.makeCurrent()) {
            // Send the message to Kafka
            String result = kafkaTemplate.send(ORDER_TOPIC, new ObjectMapper().writeValueAsString(order)).join().toString();
            log.info("Send order message " + order + ". Result " + result);
        } finally {
            span.end();
        }
    }
}
