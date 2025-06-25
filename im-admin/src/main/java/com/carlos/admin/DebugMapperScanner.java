package com.carlos.admin;

import com.carlos.admin.domain.mapper.LitemallCarWhiteMapper;
import org.springframework.beans.factory.config.BeanFactoryPostProcessor;
import org.springframework.beans.factory.config.ConfigurableListableBeanFactory;
import org.springframework.context.annotation.Configuration;

import java.util.Arrays;

@Configuration
public class DebugMapperScanner implements BeanFactoryPostProcessor {

    @Override
    public void postProcessBeanFactory(ConfigurableListableBeanFactory beanFactory) {
        String[] mapperNames = beanFactory.getBeanNamesForType(LitemallCarWhiteMapper.class);
        System.out.println("✅ LitemallCarWhiteMapper 注册情况: " + Arrays.toString(mapperNames));
    }
}

