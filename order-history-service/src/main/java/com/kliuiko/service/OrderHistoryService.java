package com.kliuiko.service;

import com.kliuiko.repository.OrderHistoryRepository;
import com.kliuiko.model.Order;
import com.kliuiko.model.OrderHistory;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.data.domain.Pageable;
import org.springframework.stereotype.Service;

import java.util.Date;
import java.util.List;

@Service
public class OrderHistoryService {

    @Autowired
    private OrderHistoryRepository orderHistoryRepository;

    public List<OrderHistory> getOrderHistories() {
        return orderHistoryRepository.findAll(Pageable.ofSize(10)).stream().toList();
    }

    public OrderHistory createOrderHistory(Order order) {
        OrderHistory orderHistory = new OrderHistory();
        orderHistory.setId(order.getId());
        Date now = new Date();
        orderHistory.setCreatedAt(now);
        orderHistory.setModifiedAt(now);
        return orderHistory;
    }
}
