package com.kliuiko.kafka;

import com.kliuiko.model.Order;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.kafka.core.KafkaTemplate;
import org.springframework.stereotype.Service;

@Service
public class OrderProducer {

    private static final String ORDER_TOPIC = "order";
    private final KafkaTemplate<String, Order> kafkaTemplate;

    @Autowired
    public OrderProducer(final KafkaTemplate<String, Order> kafkaTemplate) {
        this.kafkaTemplate = kafkaTemplate;
    }

    public void sendMessage(Order order) {
        kafkaTemplate.send(ORDER_TOPIC, order);
    }
}
