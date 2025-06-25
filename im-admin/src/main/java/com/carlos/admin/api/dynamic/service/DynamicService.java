package com.carlos.admin.api.dynamic.service;

import com.baomidou.mybatisplus.core.conditions.query.LambdaQueryWrapper;
import com.carlos.admin.config.dynamic.datasource.DynamicDataSourceContextHolder;
import com.carlos.admin.domain.LitemallCarWhite;
import com.carlos.admin.domain.mapper.LitemallCarWhiteMapper;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class DynamicService {

    private final LitemallCarWhiteMapper carWhiteMapper;

    public DynamicService(LitemallCarWhiteMapper carWhiteMapper) {
        this.carWhiteMapper = carWhiteMapper;
    }

    public void list() {
        DynamicDataSourceContextHolder.setDataSourceKey("6045edccfcdf491cc43256c1");
        this.getList();
        DynamicDataSourceContextHolder.setDataSourceKey("60780381fcdf491f24c12fff");
        this.getList();
    }

    public void getList() {
        LitemallCarWhite carWhite = this.carWhiteMapper.selectById(1);
        System.out.println(carWhite);
    }
}
