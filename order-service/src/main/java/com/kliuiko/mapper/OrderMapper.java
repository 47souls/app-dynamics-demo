package com.kliuiko.mapper;

import com.kliuiko.dto.CreateOrderDto;
import com.kliuiko.model.Order;
import org.mapstruct.Mapper;
import org.springframework.stereotype.Service;

@Mapper
@Service
public interface OrderMapper {
    Order createOrderDtoToCar(CreateOrderDto createOrderDto);
}
