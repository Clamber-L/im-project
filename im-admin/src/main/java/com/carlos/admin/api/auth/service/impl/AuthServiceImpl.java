package com.carlos.admin.api.auth.service.impl;

import com.baomidou.mybatisplus.core.conditions.query.LambdaQueryWrapper;
import com.carlos.admin.api.auth.service.AuthService;
import com.carlos.admin.api.auth.vo.LoginParam;
import com.carlos.admin.config.exception.CarlosException;
import com.carlos.admin.config.redis.RedisConstants;
import com.carlos.admin.config.redis.RedisService;
import com.carlos.admin.domain.SysUser;
import com.carlos.admin.domain.mapper.SysRoleMapper;
import com.carlos.admin.domain.mapper.SysUserMapper;
import com.carlos.admin.utils.jwt.JwtUtils;
import com.carlos.admin.utils.jwt.TokenResponse;
import io.github.clamber.excel.ExcelImport;
import io.github.clamber.excel.ExcelUtils;
import org.springframework.security.authentication.AuthenticationManager;
import org.springframework.security.authentication.UsernamePasswordAuthenticationToken;
import org.springframework.security.core.Authentication;
import org.springframework.stereotype.Service;

import java.util.concurrent.TimeUnit;

@Service
public class AuthServiceImpl implements AuthService {

	private final SysUserMapper userMapper;

	private final SysRoleMapper roleMapper;

	private final AuthenticationManager manager;

	private final RedisService redisService;

	public AuthServiceImpl(SysUserMapper userMapper, SysRoleMapper roleMapper, AuthenticationManager manager, RedisService redisService) {
		this.userMapper = userMapper;
		this.roleMapper = roleMapper;
		this.manager = manager;
		this.redisService = redisService;
	}

	@Override
	public TokenResponse login(LoginParam param) {
		UsernamePasswordAuthenticationToken authenticationToken = new UsernamePasswordAuthenticationToken(param.getAccount(), param.getPassword());

		Authentication authenticate = manager.authenticate(authenticationToken);
		if (authenticate == null) {
			throw new CarlosException("登录失败，账号或密码错误");
		}
		SysUser sysUser = (SysUser) authenticate.getPrincipal();
		TokenResponse tokenResponse = JwtUtils.createToken(sysUser.getId());
		if (tokenResponse == null) {
			throw new CarlosException("登录失败");
		}

		this.redisService.setEx(RedisConstants.Login_Token + sysUser.getId(), tokenResponse.getToken(), 2L, TimeUnit.HOURS);
		return tokenResponse;
	}

	@Override
	public void currentUser() {
		SysUser sysUser = this.userMapper.selectOne(new LambdaQueryWrapper<SysUser>().eq(SysUser::getId, "123"));
	}
}
