package com.carlos.admin.config.security;

import com.baomidou.mybatisplus.core.conditions.query.LambdaQueryWrapper;
import com.carlos.admin.config.exception.CarlosException;
import com.carlos.admin.domain.SysUser;
import com.carlos.admin.domain.mapper.SysUserMapper;
import com.carlos.admin.utils.HttpResult;
import com.carlos.admin.utils.PasswordUtils;
import com.fasterxml.jackson.databind.ObjectMapper;
import lombok.extern.slf4j.Slf4j;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.http.HttpMethod;
import org.springframework.security.authentication.AuthenticationManager;
import org.springframework.security.authentication.BadCredentialsException;
import org.springframework.security.authentication.UsernamePasswordAuthenticationToken;
import org.springframework.security.config.Customizer;
import org.springframework.security.config.annotation.web.builders.HttpSecurity;
import org.springframework.security.config.annotation.web.configurers.AbstractHttpConfigurer;
import org.springframework.security.config.annotation.web.configurers.CorsConfigurer;
import org.springframework.security.config.http.SessionCreationPolicy;
import org.springframework.security.core.Authentication;
import org.springframework.security.core.AuthenticationException;
import org.springframework.security.core.userdetails.UsernameNotFoundException;
import org.springframework.security.web.AuthenticationEntryPoint;
import org.springframework.security.web.SecurityFilterChain;
import org.springframework.security.web.authentication.UsernamePasswordAuthenticationFilter;

import java.io.PrintWriter;
import java.util.Objects;

@Slf4j
@Configuration
public class SecurityConfiguration implements AuthenticationManager {

	private final SysUserMapper userMapper;

	private final ObjectMapper objectMapper;

	private final JwtAuthenticationTokenFilter jwtAuthenticationTokenFilter;

	public SecurityConfiguration(SysUserMapper userMapper, ObjectMapper objectMapper, JwtAuthenticationTokenFilter jwtAuthenticationTokenFilter) {
		this.userMapper = userMapper;
		this.objectMapper = objectMapper;
		this.jwtAuthenticationTokenFilter = jwtAuthenticationTokenFilter;
	}

//	@Bean
//	public AuthenticationProvider authenticationProvider() {
//		return new AuthenticationProvider() {
//			@Override
//			public Authentication authenticate(Authentication authentication) throws AuthenticationException {
//				String username = authentication.getName();
//				String password = authentication.getCredentials().toString();
//				SysUser user = userMapper.selectOne(new LambdaQueryWrapper<>(SysUser.class).eq(SysUser::getAccount, username));
//
//				if (user == null) {
//					throw new UsernameNotFoundException("用户不存在");
//				}
//
//				String encryptPassword = PasswordUtils.encrypt(password, user.getSalt());
//				if (Objects.equals(user.getPassword(), encryptPassword)) {
//					log.info("login success:{}", user);
//					return new UsernamePasswordAuthenticationToken(username, password, user.getAuthorities());
//				}else {
//					throw new BadCredentialsException("密码错误");
//				}
//			}
//
//			@Override
//			public boolean supports(Class<?> authentication) {
//				return authentication.equals(UsernamePasswordAuthenticationToken.class);
//			}
//		};
//	}

	@Bean
	public AuthenticationEntryPoint authenticationEntryHandle() {
		return  (_, response, exception) -> {
			response.setContentType("application/json");
			response.setStatus(401);
			response.setCharacterEncoding("UTF-8");

			System.out.println(exception.getMessage());
			HttpResult<Object> result = HttpResult.errorNoAuth("登录失败，认证错误");
			String valueAsString = objectMapper.writeValueAsString(result);
			PrintWriter writer = response.getWriter();
			writer.write(valueAsString);
			writer.flush();
			writer.close();
		};
	}

	@Bean
	public SecurityFilterChain filterChain(HttpSecurity security) throws Exception {
//		security.sessionManagement(session ->
//				session.sessionCreationPolicy(SessionCreationPolicy.STATELESS)) 定义为无状态服务，不会根据服务器session判断登录状态
//				.authorizeHttpRequests((req)->req.anyRequest().authenticated()).httpBasic(withDefaults());
//		return security.build();
		return security.sessionManagement(session -> session.sessionCreationPolicy(SessionCreationPolicy.STATELESS)) // 定义为无状态服务，不会根据服务器session判断登录状态
				// 配置异常拦截
				.exceptionHandling(exception -> exception.authenticationEntryPoint(authenticationEntryHandle()))
				.authorizeHttpRequests(request -> request.requestMatchers(HttpMethod.POST,"/auth/login").anonymous().anyRequest().authenticated()) // 任何请求必须经过认证才允许请求
				// /auth/login 不需要对应的controller，会自动绑定到认证的 authenticate 方法
//				.formLogin(form -> form.loginProcessingUrl("/auth/login").permitAll().usernameParameter("account").passwordParameter("password"))
				// 退出配置 清除session信息和认证信息
//				.logout(logout -> logout.logoutUrl("/auth/logout").invalidateHttpSession(true).clearAuthentication(true))
				// 设置自定义过滤器 验证token
				.addFilterBefore(jwtAuthenticationTokenFilter, UsernamePasswordAuthenticationFilter.class)
				.csrf(AbstractHttpConfigurer::disable)
				// 开启允许跨域请求
				.cors(Customizer.withDefaults())
				.build();
	}

	@Override
	public Authentication authenticate(Authentication authentication) throws AuthenticationException {
		String username = authentication.getName();
		String password = authentication.getCredentials().toString();
		SysUser user = userMapper.selectOne(new LambdaQueryWrapper<>(SysUser.class).eq(SysUser::getAccount, username));

		if (user == null) {
			throw new CarlosException("用户不存在");
		}

		String encryptPassword = PasswordUtils.encrypt(password, user.getSalt());
		if (Objects.equals(user.getPassword(), encryptPassword)) {
			log.info("login success:{}", user);
			return new UsernamePasswordAuthenticationToken(user, password, user.getAuthorities());
		}else {
			throw new CarlosException("密码错误");
		}
	}
}
