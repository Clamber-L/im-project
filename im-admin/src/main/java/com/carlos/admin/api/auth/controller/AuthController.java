package com.carlos.admin.api.auth.controller;

import com.carlos.admin.api.auth.service.AuthService;
import com.carlos.admin.api.auth.vo.LoginParam;
import com.carlos.admin.utils.HttpResult;
import com.carlos.admin.utils.jwt.TokenResponse;
import com.google.common.collect.Lists;
import com.google.common.collect.Maps;
import jakarta.servlet.http.HttpServletRequest;
import org.checkerframework.checker.nullness.qual.Nullable;
import org.springframework.web.bind.annotation.*;

import java.util.HashMap;
import java.util.Map;

@RestController
@RequestMapping("/auth")
public class AuthController {

	private final AuthService service;

	public AuthController(AuthService service) {
		this.service = service;
	}

	@PostMapping("login")
	public HttpResult<TokenResponse> login(@RequestBody LoginParam param) {
		return HttpResult.ok(this.service.login(param));
	}

	@GetMapping("currentUser")
	public HttpResult<Map<String, Object>> currentUser(HttpServletRequest request) {
		HashMap<@Nullable String, @Nullable Object> map = Maps.newHashMap();
		map.put("username","123");
		map.put("avatar","avatar.jpg");
		map.put("role","admin");
		map.put("rules", Lists.newArrayList("*"));
		this.service.currentUser();
		return HttpResult.ok(map);
	}
}
