package com.carlos.admin.config.security;

import com.carlos.admin.utils.jwt.UserBean;
import com.carlos.admin.config.redis.RedisConstants;
import com.carlos.admin.config.redis.RedisService;
import com.carlos.admin.domain.SysUser;
import com.carlos.admin.domain.mapper.SysUserMapper;
import com.carlos.admin.utils.jwt.JwtUtils;
import jakarta.servlet.FilterChain;
import jakarta.servlet.ServletException;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;
import org.springframework.security.authentication.UsernamePasswordAuthenticationToken;
import org.springframework.security.core.context.SecurityContextHolder;
import org.springframework.stereotype.Component;
import org.springframework.util.StringUtils;
import org.springframework.web.filter.OncePerRequestFilter;

import java.io.IOException;

@Component
public class JwtAuthenticationTokenFilter extends OncePerRequestFilter {

	private final RedisService redisService;

	private final SysUserMapper userMapper;

	static final String HEADER_TOKEN = "token";

	public JwtAuthenticationTokenFilter(RedisService redisService, SysUserMapper userMapper) {
		this.redisService = redisService;
		this.userMapper = userMapper;
	}

	@Override
	protected void doFilterInternal(HttpServletRequest request, HttpServletResponse response, FilterChain filterChain) throws ServletException, IOException {
		String token = request.getHeader(HEADER_TOKEN);

		if (!StringUtils.hasText(token)) {
			filterChain.doFilter(request, response);
			return;
		}

		UserBean userBean = JwtUtils.verifyTokenAndGetUserBean(token);
		boolean existsKey = this.redisService.existsKey(RedisConstants.Login_Token + userBean.getUserId());
		if (!existsKey) {
			filterChain.doFilter(request, response);
			return;
		}
		SysUser user = this.userMapper.selectById(userBean.getUserId());
		UsernamePasswordAuthenticationToken authenticationToken = new UsernamePasswordAuthenticationToken(user, null, null);
		SecurityContextHolder.getContext().setAuthentication(authenticationToken);
		filterChain.doFilter(request, response);
	}
}
