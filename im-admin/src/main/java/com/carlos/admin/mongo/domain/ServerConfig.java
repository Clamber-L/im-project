package com.carlos.admin.mongo.domain;

import lombok.Data;
import org.springframework.data.annotation.Id;
import org.springframework.data.mongodb.core.mapping.Document;
import org.springframework.data.mongodb.core.mapping.Field;

import java.io.Serializable;
import java.util.Date;

/**
 * ServerConfig class
 *
 *
 */
@Document(collection = "ServerConfig")
@Data
public class ServerConfig implements Serializable {
    @Id
    private String id;

    @Field("server_ip")
    private String serverIp;

    @Field("user_name")
    private String userName;

    @Field("pass")
    private String pass;

    @Field("port")
    private int port;

    @Field("dbname")
    private String dbname;

    @Field("status")
    private int status;

    @Field("total_size")
    private int total_size;

    @Field("use_size")
    private int use_size;

    @Field("add_time")
    private Date addTime;

    @Field("update_time")
    private Date updateTime;
}
