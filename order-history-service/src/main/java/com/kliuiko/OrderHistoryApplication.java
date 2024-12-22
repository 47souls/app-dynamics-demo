package com.kliuiko;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.context.annotation.EnableAspectJAutoProxy;

@SpringBootApplication
@EnableAspectJAutoProxy
public class OrderHistoryApplication {
	public static void main(String[] args) {
		SpringApplication.run(OrderHistoryApplication.class, args);
	}
}
