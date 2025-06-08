package com.carlos.imflink;

import org.apache.kafka.clients.admin.*;
import org.apache.kafka.clients.consumer.ConsumerConfig;
import org.apache.kafka.common.errors.TopicExistsException;
import org.apache.kafka.common.serialization.StringDeserializer;

import java.util.ArrayList;
import java.util.Collections;
import java.util.Map;
import java.util.Properties;
import java.util.concurrent.ExecutionException;

public class KafkaAdmin {

//    public static void main(String[] args) throws ExecutionException, InterruptedException {
//        // Kafka 集群配置
//        String bootstrapServers = "192.168.0.25:9093";
//
//        // 创建一个新的 Topic
//        try (AdminClient adminClient = AdminClient.create(Collections.singletonMap("bootstrap.servers", bootstrapServers))) {
//            NewTopic newTopic = new NewTopic("my_topic", 3, (short) 1); // 3 个分区，2 个副本
//            CreateTopicsResult result = adminClient.createTopics(Collections.singleton(newTopic));
//            result.all().get(); // 等待创建完成
//            System.out.println("Topic created successfully!");
//        } catch (TopicExistsException e) {
//            System.out.println("Topic already exists.");
//        }
//    }

    public static void main(String[] args) throws ExecutionException, InterruptedException {
        Properties props = new Properties();
        props.put(ConsumerConfig.BOOTSTRAP_SERVERS_CONFIG, "192.168.0.25:9093");  // Kafka 集群地址
        props.put(ConsumerConfig.GROUP_ID_CONFIG, "my-consumer-group");  // 消费者组ID
        props.put(ConsumerConfig.KEY_DESERIALIZER_CLASS_CONFIG, StringDeserializer.class.getName());  // 键反序列化器
        props.put(ConsumerConfig.VALUE_DESERIALIZER_CLASS_CONFIG, StringDeserializer.class.getName());  // 值反序列化器
        props.put(ConsumerConfig.AUTO_OFFSET_RESET_CONFIG, "earliest");  // 如果没有初始偏移量，则从最早的消息开始消费

        // 创建 AdminClient 实例
        AdminClient adminClient = AdminClient.create(props);

        // 获取 Topic 的详细描述
        ArrayList<String> objects = new ArrayList<>();
        objects.add("my_topic");

        DescribeTopicsResult result = adminClient.describeTopics(objects);
        Map<String, TopicDescription> descriptionMap = result.all().get();
        TopicDescription topicDescription = descriptionMap.get("my_topic");

        // 输出分区信息
        System.out.println("Partitions for 'my_topic': " + topicDescription.partitions());
    }
}
