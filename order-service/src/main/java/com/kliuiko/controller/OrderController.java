package com.kliuiko.controller;

import com.kliuiko.dto.CreateOrderDto;
import com.kliuiko.model.Order;
import com.kliuiko.service.OrderService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RestController;

@RestController
public class OrderController {

    private final OrderService orderService;

    @Autowired
    public OrderController(final OrderService orderService) {
        this.orderService = orderService;
    }

    @PostMapping("/order")
    public Order createOrder(@RequestBody CreateOrderDto createOrderDTO) {
        return orderService.createOrder(createOrderDTO);
    }
}
