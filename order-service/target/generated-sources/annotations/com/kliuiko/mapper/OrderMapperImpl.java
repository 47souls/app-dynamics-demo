package com.kliuiko.mapper;

import com.kliuiko.dto.CreateOrderDto;
import com.kliuiko.model.Order;
import javax.annotation.processing.Generated;

@Generated(
    value = "org.mapstruct.ap.MappingProcessor",
    date = "2024-11-17T15:39:01+0200",
    comments = "version: 1.6.3, compiler: javac, environment: Java 18.0.2 (Amazon.com Inc.)"
)
public class OrderMapperImpl implements OrderMapper {

    @Override
    public Order createOrderDtoToCar(CreateOrderDto createOrderDto) {
        if ( createOrderDto == null ) {
            return null;
        }

        Order order = new Order();

        return order;
    }
}
