package com.kliuiko.model;

import jakarta.persistence.Entity;
import jakarta.persistence.Id;

import java.util.UUID;

@Entity
public class Order {

    @Id
    private UUID id;
}
