package com.kliuiko.kafka;

import com.fasterxml.jackson.databind.ObjectMapper;
import com.kliuiko.model.Order;
import lombok.Getter;
import lombok.SneakyThrows;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.kafka.core.KafkaTemplate;
import org.springframework.stereotype.Service;

@Service
@Slf4j
public class OrderProducer {

    @Getter
    private static final String ORDER_TOPIC = "order";
    private final KafkaTemplate<String, String> kafkaTemplate;

    @Autowired
    public OrderProducer(final KafkaTemplate<String, String> kafkaTemplate) {
        this.kafkaTemplate = kafkaTemplate;
    }

    @SneakyThrows
    public void sendMessage(Order order) {
        String result = kafkaTemplate.send(ORDER_TOPIC, new ObjectMapper().writeValueAsString(order)).join().toString();
        log.info("Send order message " + order + ". Result " + result);
    }
}
