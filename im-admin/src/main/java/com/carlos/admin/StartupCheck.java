package com.carlos.admin;

import org.apache.ibatis.session.SqlSessionFactory;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.ApplicationArguments;
import org.springframework.boot.ApplicationRunner;
import org.springframework.context.annotation.Configuration;

@Configuration
public class StartupCheck implements ApplicationRunner {

    @Autowired
    private SqlSessionFactory sqlSessionFactory;

    @Override
    public void run(ApplicationArguments args) {
        System.out.println("================= MyBatis Mapped Statements =================");
        sqlSessionFactory.getConfiguration().getMappedStatementNames().forEach(System.out::println);
        System.out.println("=============================================================");
        System.out.println("✅ 当前 SqlSessionFactory 类型: " + sqlSessionFactory.getClass().getName());
        System.out.println("✅ 当前 MappedStatement 总数: " + sqlSessionFactory.getConfiguration().getMappedStatementNames().size());

    }
}
