package com.kliuiko.controller;

import com.kliuiko.model.OrderHistory;
import com.kliuiko.service.OrderHistoryService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.List;

@RestController
public class OrderHistoryController {

    private final OrderHistoryService orderHistoryService;

    @Autowired
    public OrderHistoryController(final OrderHistoryService orderHistoryService) {
        this.orderHistoryService = orderHistoryService;
    }

    @GetMapping("/order-history")
    public List<OrderHistory> getOrderHistories() {
        return orderHistoryService.getOrderHistories();
    }
}
