package com.carlos.admin.config.dynamic.datasource;

import java.util.ArrayList;
import java.util.Collection;
import java.util.List;

public class DynamicDataSourceContextHolder {

    private static final ThreadLocal<String> contextHolder = new ThreadLocal<>();

    /**
     * 数据源的 key集合，用于切换时判断数据源是否存在
     */
    public static List<Object> dataSourceKeys = new ArrayList<>();

    /**
     * 切换数据源
     * @param key
     */
    public static void setDataSourceKey(String key) {
        contextHolder.set(key);
    }

    /**
     * 获取当前数据源
     * @return
     */
    public static String getDataSourceKey() {
        return contextHolder.get();
    }

    /**
     * 清除数据源
     */
    public static void clearDataSourceKey() {
        contextHolder.remove();
    }

    /**
     * 添加数据源keys
     *
     * @param keys
     */
    public static void addDataSourceKeys(Collection<?> keys) {
        dataSourceKeys.addAll(keys);
    }
}
