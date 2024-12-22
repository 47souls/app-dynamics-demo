package com.kliuiko.service;

import com.kliuiko.dto.CreateOrderDto;
import com.kliuiko.mapper.OrderMapper;
import com.kliuiko.model.Order;
import com.kliuiko.repository.OrderRepository;
import com.kliuiko.kafka.OrderProducer;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

@Service
public class OrderService {

    private final OrderProducer orderProducer;
    private final OrderRepository orderRepository;
    private final OrderMapper orderMapper;

    @Autowired
    public OrderService(final OrderRepository orderRepository,
                        final OrderMapper orderMapper,
                        final OrderProducer orderProducer) {
        this.orderRepository = orderRepository;
        this.orderMapper = orderMapper;
        this.orderProducer = orderProducer;
    }

    public Order createOrder(CreateOrderDto createOrderDTO) {
        Order orderToCreate = orderMapper.createOrderDtoToCar(createOrderDTO);
        Order createdOrder = orderRepository.save(orderToCreate);
        orderProducer.sendMessage(createdOrder);
        return createdOrder;
    }
}
