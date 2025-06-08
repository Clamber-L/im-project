package com.carlos.imflink;

import org.apache.kafka.clients.producer.KafkaProducer;
import org.apache.kafka.clients.producer.ProducerConfig;
import org.apache.kafka.clients.producer.ProducerRecord;
import org.apache.kafka.common.serialization.StringSerializer;

import java.util.Properties;

public class KafkaProduct {

    public static void main(String[] args) {
        // 配置生产者的属性
        Properties props = new Properties();
        props.put(ProducerConfig.BOOTSTRAP_SERVERS_CONFIG, "192.168.0.25:9093");  // Kafka 集群地址
        props.put(ProducerConfig.KEY_SERIALIZER_CLASS_CONFIG, StringSerializer.class.getName());  // 键序列化器
        props.put(ProducerConfig.VALUE_SERIALIZER_CLASS_CONFIG, StringSerializer.class.getName());  // 值序列化器

        // 创建 KafkaProducer 实例
        KafkaProducer<String, String> producer = new KafkaProducer<>(props);

        // 向特定 Topic 和 Partition 发送消息
        String topic = "my_topic";

        for (int i = 0; i < 15; i++) {
            int partition = i % 3;  // 指定发送到的分区
            String message = "Hello, Kafka!" + partition;
            // 创建消息并指定 Topic 和 Partition
            ProducerRecord<String, String> record = new ProducerRecord<>(topic, partition, "key", message);

            try {
                // 发送消息
                producer.send(record, (metadata, exception) -> {
                    if (exception != null) {
                        exception.printStackTrace();
                    } else {
                        System.out.println("Message sent to topic " + metadata.topic() + " partition " + metadata.partition());
                    }
                });
            } catch (Exception e) {
                e.printStackTrace();
            }
        }
        producer.close();  // 关闭生产者

    }
}
