package com.carlos.imflink;

import org.apache.kafka.clients.consumer.ConsumerConfig;
import org.apache.kafka.clients.consumer.ConsumerRecord;
import org.apache.kafka.clients.consumer.ConsumerRecords;
import org.apache.kafka.clients.consumer.KafkaConsumer;
import org.apache.kafka.common.serialization.StringDeserializer;

import java.util.Arrays;
import java.util.Properties;

public class KafkaConsume {

    public static void main(String[] args) {
        // 配置 Kafka 消费者
        Properties props = new Properties();
        props.put(ConsumerConfig.BOOTSTRAP_SERVERS_CONFIG, "192.168.0.25:9093");  // Kafka 集群地址
        props.put(ConsumerConfig.GROUP_ID_CONFIG, "my-consumer-group");  // 消费者组ID
        props.put(ConsumerConfig.KEY_DESERIALIZER_CLASS_CONFIG, StringDeserializer.class.getName());  // 键反序列化器
        props.put(ConsumerConfig.VALUE_DESERIALIZER_CLASS_CONFIG, StringDeserializer.class.getName());  // 值反序列化器
        props.put(ConsumerConfig.AUTO_OFFSET_RESET_CONFIG, "earliest");  // 如果没有初始偏移量，则从最早的消息开始消费

        // 创建 Kafka 消费者实例
        KafkaConsumer<String, String> consumer = new KafkaConsumer<>(props);

        // 指定 Topic 和 Partition，进行订阅
        String topic = "my_topic";
        int partition0 = 0;
        int partition1 = 1;
        int partition2 = 2;

        // 创建 Partition 对象
        consumer.assign(Arrays.asList(
                new org.apache.kafka.common.TopicPartition(topic, partition0),
                new org.apache.kafka.common.TopicPartition(topic, partition1),
                new org.apache.kafka.common.TopicPartition(topic, partition2)
        ));

        // 持续拉取消息
        while (true) {
            // 拉取消息
            ConsumerRecords<String, String> records = consumer.poll(1000);// 每次最多等待 1000 毫秒，拉取一批消息

            for (ConsumerRecord<String, String> record : records) {
                System.out.println("Consumed message: " + record.value() +
                        " from partition " + record.partition() +
                        " at offset " + record.offset());
            }
        }
    }
}
