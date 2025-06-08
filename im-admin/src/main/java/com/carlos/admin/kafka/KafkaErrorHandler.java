package com.carlos.admin.kafka;

import org.springframework.context.annotation.Bean;
import org.springframework.kafka.core.KafkaTemplate;
import org.springframework.kafka.listener.ConsumerRecordRecoverer;
import org.springframework.kafka.listener.DeadLetterPublishingRecoverer;
import org.springframework.kafka.listener.DefaultErrorHandler;
import org.springframework.stereotype.Component;
import org.springframework.util.backoff.BackOff;
import org.springframework.util.backoff.FixedBackOff;

@Component
public class KafkaErrorHandler {

//    private final KafkaTemplate<Object, Object> kafkaTemplate;
//
//    public KafkaErrorHandler(KafkaTemplate<Object, Object> kafkaTemplate) {
//        this.kafkaTemplate = kafkaTemplate;
//    }
//
//    @Bean
//    public DefaultErrorHandler errorHandler() {
//        ConsumerRecordRecoverer recoverer = new DeadLetterPublishingRecoverer(kafkaTemplate);
//        // 设置重试间隔 10秒 次数为 3次
//        BackOff backOff = new FixedBackOff(10 * 1000L, 3L);
//        // 创建 DefaultErrorHandler 对象
//        return new DefaultErrorHandler(recoverer, backOff);
//    }
}
