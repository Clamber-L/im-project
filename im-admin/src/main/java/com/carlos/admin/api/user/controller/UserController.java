package com.carlos.admin.api.user.controller;

import com.carlos.admin.api.user.service.UserService;
import com.carlos.admin.api.user.vo.param.DeleteUserParam;
import com.carlos.admin.domain.SysUser;
import com.carlos.admin.utils.HttpResult;
import com.carlos.admin.utils.page.PageRequest;
import com.carlos.admin.utils.page.PageResult;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.tags.Tag;
import org.springframework.validation.annotation.Validated;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@Validated
@RestController
@RequestMapping("/user")
@Tag(name = "用户管理")
public class UserController {

	private final UserService service;

	public UserController(UserService service) {
		this.service = service;
	}

	@Operation(summary = "用户信息")
	@GetMapping("userInfo")
	public HttpResult<SysUser> userInfo() {
		this.service.userInfo();
		return HttpResult.ok();
	}

	@Operation(summary = "保存用户")
	@PostMapping("saveUser")
	public HttpResult<Void> saveUser(@RequestBody SysUser user) {
		this.service.saveUser(user);
		return HttpResult.ok();
	}

	@Operation(summary = "删除用户")
	@PostMapping("deleteUser")
	public HttpResult<Void> deleteUser(@RequestBody DeleteUserParam param) {
		this.service.deleteUser(param);
		return HttpResult.ok();
	}

	@Operation(summary = "用户列表")
	@GetMapping("list")
	public HttpResult<PageResult<SysUser>> list() {
		PageRequest.startPage();
		List<SysUser> list = this.service.list();
		return HttpResult.ok(PageResult.parse(list));
	}

	@Operation(summary = "可选择的")
	@PostMapping("selective")
	public HttpResult<Void> selective() {
		this.service.selective();
		return HttpResult.ok();
	}
}
