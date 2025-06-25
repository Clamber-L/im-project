package com.carlos.admin;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.cache.annotation.EnableCaching;
import org.springframework.kafka.annotation.EnableKafka;

@EnableKafka
@EnableCaching
@SpringBootApplication
public class ImAdminApplication {

	public static void main(String[] args) {
		SpringApplication.run(ImAdminApplication.class, args);
	}
}
