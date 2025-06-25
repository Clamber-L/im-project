package com.carlos.admin.mongo.service;

import com.carlos.admin.mongo.domain.ServerConfig;
import org.springframework.data.mongodb.core.MongoTemplate;
import org.springframework.stereotype.Component;

@Component
public class ServerConfigService {

    private final MongoTemplate mongoTemplate;

    public ServerConfigService(MongoTemplate mongoTemplate) {
        this.mongoTemplate = mongoTemplate;
    }

    public ServerConfig findById(String id) {
        return mongoTemplate.findById(id, ServerConfig.class);
    }
}
