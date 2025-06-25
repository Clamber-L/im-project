package com.carlos.admin.api.dynamic.controller;

import com.carlos.admin.api.dynamic.service.DynamicService;
import com.carlos.admin.utils.HttpResult;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("dynamic")
public class DynamicController {

    private final DynamicService service;

    public DynamicController(DynamicService service) {
        this.service = service;
    }

    @GetMapping("list")
    public HttpResult<Void> list() {
        this.service.list();
        return HttpResult.ok();
    }
}
