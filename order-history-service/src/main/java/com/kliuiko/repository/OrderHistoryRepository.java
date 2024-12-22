package com.kliuiko.repository;

import com.kliuiko.model.OrderHistory;
import org.springframework.data.repository.CrudRepository;
import org.springframework.stereotype.Repository;

@Repository
public interface OrderHistoryRepository extends CrudRepository<OrderHistory, Long> {

}
