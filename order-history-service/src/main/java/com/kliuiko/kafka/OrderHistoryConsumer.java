package com.kliuiko.kafka;

import com.kliuiko.model.OrderHistory;
import com.kliuiko.service.OrderHistoryService;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.kafka.annotation.KafkaListener;
import org.springframework.kafka.support.Acknowledgment;
import org.springframework.stereotype.Service;

@Service
public class OrderHistoryConsumer {

    private final Logger logger = LoggerFactory.getLogger(OrderHistoryConsumer.class);

    private final OrderHistoryService orderHistoryService;

    @Autowired
    public OrderHistoryConsumer(final OrderHistoryService orderHistoryService) {
        this.orderHistoryService = orderHistoryService;
    }

    @KafkaListener(topics = "order", groupId = "order-group")
    public void consume(String order, Acknowledgment ack) throws InterruptedException {
        logger.info("Received order message: " + order + " , creating order history, it takes time (3s)");
        OrderHistory orderHistory = orderHistoryService.createOrderHistory(order);
        logger.info("Order history for order with id " + orderHistory.getId() + " was created. Created at " + orderHistory.getCreatedAt());
        ack.acknowledge();
    }
}
