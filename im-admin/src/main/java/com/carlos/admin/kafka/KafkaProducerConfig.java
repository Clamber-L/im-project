package com.carlos.admin.kafka;

import org.apache.kafka.clients.producer.ProducerConfig;
import org.apache.kafka.common.serialization.StringSerializer;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.kafka.core.DefaultKafkaProducerFactory;
import org.springframework.kafka.core.KafkaTemplate;
import org.springframework.kafka.core.ProducerFactory;

import java.util.HashMap;
import java.util.Map;

@Configuration
public class KafkaProducerConfig {

    @Bean
    public KafkaTemplate<String, String> kafkaTemplate() {
        return new KafkaTemplate<>(producerFactory());
    }

    @Bean
    public ProducerFactory<String, String> producerFactory() {
        Map<String, Object> configProps = new HashMap<>();
        configProps.put(ProducerConfig.BOOTSTRAP_SERVERS_CONFIG, "192.168.0.25:9093");
        configProps.put(ProducerConfig.KEY_SERIALIZER_CLASS_CONFIG, StringSerializer.class);
        configProps.put(ProducerConfig.VALUE_SERIALIZER_CLASS_CONFIG, StringSerializer.class);
        configProps.put(ProducerConfig.ACKS_CONFIG, "all");  // 确保消息已同步到所有副本
        configProps.put(ProducerConfig.RETRIES_CONFIG, 3);   // 设置重试次数
        configProps.put(ProducerConfig.LINGER_MS_CONFIG, 1);  // 控制发送批次 延迟时间
        configProps.put(ProducerConfig.BATCH_SIZE_CONFIG, 16384);  // 设置批次大小
        configProps.put(ProducerConfig.DELIVERY_TIMEOUT_MS_CONFIG, 3500 * 10);  // 设置超时时间 可能造成重复投递问题，需要设置幂等
        configProps.put(ProducerConfig.ENABLE_IDEMPOTENCE_CONFIG, true);  // 设置幂等性

        return new DefaultKafkaProducerFactory<>(configProps);
    }
}