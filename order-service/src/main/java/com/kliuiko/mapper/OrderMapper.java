package com.kliuiko.mapper;

import com.kliuiko.dto.CreateOrderDto;
import com.kliuiko.model.Order;
import org.springframework.stereotype.Service;

@Service
public class OrderMapper {
    public Order createOrderDtoToCar(CreateOrderDto createOrderDto) {
        Order order = new Order();
        order.setOrderDescription(createOrderDto.getOrderDescription());
        return order;
    };
}
