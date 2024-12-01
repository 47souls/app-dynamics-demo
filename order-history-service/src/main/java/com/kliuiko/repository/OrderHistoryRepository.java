package com.kliuiko.repository;

import com.kliuiko.model.OrderHistory;
import org.springframework.data.repository.PagingAndSortingRepository;
import org.springframework.stereotype.Repository;

@Repository
public interface OrderHistoryRepository extends PagingAndSortingRepository<OrderHistory, Long> {

}
