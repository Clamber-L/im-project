package com.carlos.admin.kafka;

import org.springframework.kafka.annotation.KafkaListener;
import org.springframework.kafka.annotation.TopicPartition;
import org.springframework.stereotype.Component;

@Component
public class MyKafkaListener {

//	@KafkaListener(
//			topics = "test_topic",  // 监听的 Kafka 主题
//			groupId = "demo01-consumer-group-1"  // 消费者组
//	)

	// kafka-topics.sh --alter --topic test_topic --partitions 3 --bootstrap-server 127.0.0.1:9092
	@KafkaListener(topicPartitions = @TopicPartition(topic = "test_topic", partitions = {"1"}), groupId = "group1")
	public void listen(String message) {
		try {
			System.out.println("Processing message: " + message);
			// 如果处理失败，抛出异常，触发重试
			if (message.contains("fail")) {
				throw new RuntimeException("Intentional failure to test retry mechanism");
			}
			System.out.println("Message processed successfully");
		} catch (Exception e) {
			System.out.println("Message failed to process, will retry: " + e.getMessage());
			throw e;  // 如果消息处理失败，抛出异常以触发重试
		}
	}

	@KafkaListener(topicPartitions = @TopicPartition(topic = "test_topic", partitions = {"2"}), groupId = "group1")
	public void listen2(String message) {
		try {
			System.out.println("Processing message123: " + message);
			// 如果处理失败，抛出异常，触发重试
			if (message.contains("fail")) {
				throw new RuntimeException("Intentional failure to test retry mechanism");
			}
			System.out.println("Message processed successfully");
		} catch (Exception e) {
			System.out.println("Message failed to process, will retry: " + e.getMessage());
			throw e;  // 如果消息处理失败，抛出异常以触发重试
		}
	}

	// 死信队列监听器
	@KafkaListener(
			topics = "test_topic-DLT",  // 死信队列主题
			groupId = "demo01-consumer-group-dlt"  // 消费者组
	)
	public void listenDLT(String message) {
		System.out.println("Message sent to DLT: " + message);
	}
}