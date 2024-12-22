package com.kliuiko.service;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.kliuiko.repository.OrderHistoryRepository;
import com.kliuiko.model.Order;
import com.kliuiko.model.OrderHistory;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.Date;

@Service
@Slf4j
public class OrderHistoryService {

    private final OrderHistoryRepository orderHistoryRepository;

    @Autowired
    public OrderHistoryService(OrderHistoryRepository orderHistoryRepository) {
        this.orderHistoryRepository = orderHistoryRepository;
    }

    public Iterable<OrderHistory> getOrderHistories() {
        return orderHistoryRepository.findAll();
    }

    public OrderHistory createOrderHistory(String stringOrder) throws InterruptedException {
        // simulate long-running operation
        Thread.sleep(3000);
        Order order;
        try {
            order = new ObjectMapper().readValue(stringOrder, Order.class);
        } catch (JsonProcessingException e) {
            throw new RuntimeException(e);
        }
        OrderHistory orderHistory = new OrderHistory();
        orderHistory.setId(order.getId());
        Date now = new Date();
        orderHistory.setCreatedAt(now);
        orderHistory.setModifiedAt(now);
        return orderHistoryRepository.save(orderHistory);
    }
}
