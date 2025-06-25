package com.carlos.admin.config;

import com.baomidou.mybatisplus.core.MybatisConfiguration;
import com.baomidou.mybatisplus.extension.plugins.MybatisPlusInterceptor;
import com.baomidou.mybatisplus.extension.spring.MybatisSqlSessionFactoryBean;
import com.carlos.admin.config.dynamic.datasource.DynamicDatasource;
import com.carlos.admin.mongo.service.ServerConfigService;
import org.apache.ibatis.plugin.Interceptor;
import org.apache.ibatis.session.SqlSessionFactory;
import org.mybatis.spring.SqlSessionFactoryBean;
import org.mybatis.spring.annotation.MapperScan;
import org.springframework.beans.factory.annotation.Qualifier;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.core.io.support.PathMatchingResourcePatternResolver;
import org.springframework.jdbc.datasource.DataSourceTransactionManager;
import org.springframework.transaction.PlatformTransactionManager;

import javax.sql.DataSource;
import java.util.HashMap;
import java.util.Map;

@Configuration
@MapperScan("com.carlos.admin.domain.mapper")
public class MybatisConfig {

    private final ServerConfigService serverConfigService;

    public MybatisConfig(ServerConfigService serverConfigService) {
        this.serverConfigService = serverConfigService;
    }

    //动态数据源
    @Bean("dynamicDataSource")
    public DataSource dynamicDataSource() {
        Map<Object, Object> dataSourceMap = new HashMap<>();
        return new DynamicDatasource(serverConfigService, dataSourceMap);
    }

    @Bean
    public SqlSessionFactory sqlSessionFactory(@Qualifier("dynamicDataSource") DataSource dataSource) throws Exception {
        // ✅ 关键：改为 MyBatis-Plus 提供的工厂
        MybatisSqlSessionFactoryBean factoryBean = new MybatisSqlSessionFactoryBean();
        factoryBean.setDataSource(dataSource);

        factoryBean.setTypeAliasesPackage("com.carlos.admin.domain");

        PathMatchingResourcePatternResolver resolver = new PathMatchingResourcePatternResolver();
        factoryBean.setMapperLocations(resolver.getResources("classpath:/mappers/*.xml"));

        // MyBatis 配置
        MybatisConfiguration configuration = new MybatisConfiguration();
        configuration.setMapUnderscoreToCamelCase(true);
        factoryBean.setConfiguration(configuration);

        // MyBatis-Plus 插件
        MybatisPlusInterceptor interceptor = new MybatisPlusInterceptor();
        factoryBean.setPlugins(new Interceptor[]{interceptor});

        return factoryBean.getObject(); // ✅ 一定要调用 getObject()
    }


    // 事务管理器也要用动态数据源
    @Bean
    public PlatformTransactionManager transactionManager(@Qualifier("dynamicDataSource") DataSource dynamicDataSource) {
        return new DataSourceTransactionManager(dynamicDataSource);
    }
}
