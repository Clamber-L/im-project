package com.carlos.admin.config.dynamic.datasource;

import com.carlos.admin.config.exception.CarlosException;
import com.carlos.admin.mongo.domain.ServerConfig;
import com.carlos.admin.mongo.service.ServerConfigService;
import com.zaxxer.hikari.HikariDataSource;
import lombok.extern.slf4j.Slf4j;
import org.springframework.jdbc.datasource.lookup.AbstractRoutingDataSource;

import javax.sql.DataSource;
import java.util.Map;

@Slf4j
public class DynamicDatasource extends AbstractRoutingDataSource {

    private final ServerConfigService serverConfigService;

    private final Map<Object, Object> dataSources;

    public DynamicDatasource(ServerConfigService serverConfigService, Map<Object, Object> dataSources) {
        this.serverConfigService = serverConfigService;
        this.dataSources = dataSources;

        super.setTargetDataSources(dataSources);
        // 将数据源的 key 放到数据源上下文的 key 集合中，用于切换时判断数据源是否有效
        DynamicDataSourceContextHolder.addDataSourceKeys(dataSources.keySet());
    }

    @Override
    protected DataSource determineTargetDataSource() {
        String key = DynamicDataSourceContextHolder.getDataSourceKey();
        if (key != null && !dataSources.containsKey(key)) {
            // 从MongoDB获取ServerConfig
            var serverConfig = serverConfigService.findById(key);
            log.info("serverConfig: {}", serverConfig);
            if (serverConfig == null) {
                log.error("ServerConfig not found for key: {}", key);
                throw new CarlosException("获取数据失败，请稍后重试!");
            }

            HikariDataSource ds = getHikariDataSource(serverConfig);
            dataSources.put(key, ds);
            // 刷新 AbstractRoutingDataSource 的数据源映射
            super.setTargetDataSources(dataSources);
            super.afterPropertiesSet();
            DynamicDataSourceContextHolder.setDataSourceKey(key);
        }
        return super.determineTargetDataSource();
    }

    private static HikariDataSource getHikariDataSource(ServerConfig serverConfig) {
        String jdbcUrl = String.format("jdbc:mysql://%s:%d/%s?useSSL=false&serverTimezone=UTC&characterEncoding=utf8", serverConfig.getServerIp(), serverConfig.getPort(), serverConfig.getDbname());
        HikariDataSource ds = new HikariDataSource();
        ds.setDriverClassName("com.mysql.cj.jdbc.Driver");
        ds.setJdbcUrl(jdbcUrl);
        ds.setUsername(serverConfig.getUserName());
        ds.setPassword(serverConfig.getPass());
        ds.setMaximumPoolSize(10); // 可按需配置
        return ds;
    }

    @Override
    protected Object determineCurrentLookupKey() {
        return DynamicDataSourceContextHolder.getDataSourceKey();
    }
}
