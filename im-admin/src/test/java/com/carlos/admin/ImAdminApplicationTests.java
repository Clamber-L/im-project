package com.carlos.admin;

import org.junit.jupiter.api.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.kafka.core.KafkaTemplate;

@SpringBootTest
class ImAdminApplicationTests {

	@Autowired
	private KafkaTemplate<String, String> kafkaTemplate;

	@Test
	void contextLoads() {
		for (int i = 0; i < 100; i++) {
			kafkaTemplate.send("test_topic",  2 , "partition1", STR."Hello, Kafka!\{i}");
		}
	}
}
