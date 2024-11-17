package com.kliuiko.service;

import com.kliuiko.dto.CreateOrderDTO;
import com.kliuiko.model.Order;
import com.kliuiko.repository.OrderRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

@Service
public class OrderService {

    private final OrderRepository orderRepository;

    @Autowired
    public OrderService(final OrderRepository orderRepository) {
        this.orderRepository = orderRepository;
    }

    public Order createOrder(CreateOrderDTO createOrderDTO) {
//        Order orderToCreate =
//        return orderRepository.save(orderToCreate);
        return new Order();
    }
}
