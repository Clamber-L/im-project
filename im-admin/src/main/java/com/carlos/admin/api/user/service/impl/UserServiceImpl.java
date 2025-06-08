package com.carlos.admin.api.user.service.impl;

import com.baomidou.mybatisplus.core.conditions.query.LambdaQueryWrapper;
import com.carlos.admin.api.user.service.UserService;
import com.carlos.admin.api.user.vo.param.DeleteUserParam;
import com.carlos.admin.config.exception.CarlosException;
import com.carlos.admin.domain.SysUser;
import com.carlos.admin.domain.mapper.SysUserMapper;
import com.carlos.admin.utils.PasswordUtils;
import lombok.extern.slf4j.Slf4j;
import org.springframework.security.core.userdetails.UserDetails;
import org.springframework.security.core.userdetails.UserDetailsService;
import org.springframework.security.core.userdetails.UsernameNotFoundException;
import org.springframework.stereotype.Service;
import org.springframework.util.StringUtils;

import java.time.LocalDateTime;
import java.util.List;
import java.util.Objects;

@Slf4j
@Service
public class UserServiceImpl implements UserService, UserDetailsService {

	private final SysUserMapper userMapper;

	public UserServiceImpl(SysUserMapper userMapper) {
		this.userMapper = userMapper;
	}

	@Override
	public UserDetails loadUserByUsername(String username) throws UsernameNotFoundException {
		SysUser user = this.userMapper.selectOne(new LambdaQueryWrapper<>(SysUser.class).eq(SysUser::getUsername, username));
		if (user == null) {
			throw new UsernameNotFoundException("用户不存在");
		}
		return user;
	}

	@Override
	public void userInfo() {

	}

	@Override
	public void saveUser(SysUser user) {
		if (!StringUtils.hasLength(user.getAccount()) || !StringUtils.hasLength(user.getUsername()) || !StringUtils.hasLength(user.getPassword())) {
			throw new CarlosException("参数错误");
		}
		log.info("param:{}", user);
		if (!StringUtils.hasLength(user.getId())) {
			// 新增
			// 判断账号是否重复
			SysUser sysUser = this.userMapper.selectOne(new LambdaQueryWrapper<>(SysUser.class).eq(SysUser::getAccount, user.getAccount()));
			if (sysUser != null) {
				throw new CarlosException("账号重复，添加失败");
			}

			String salt = PasswordUtils.randomGen(8);
			String password = PasswordUtils.encrypt(user.getPassword(), salt);
			user.setSalt(salt);
			user.setPassword(password);
			user.setBeSuper(false);
			user.setCreatedTime(LocalDateTime.now());
			this.userMapper.insert(user);
		}else {
			// 判断两次密码是否相同
			SysUser sysUser = this.userMapper.selectById(user.getId());
			String password = PasswordUtils.encrypt(sysUser.getPassword(), sysUser.getSalt());

			if (!Objects.equals(password, user.getPassword())) {
				// 两次输入密码不一致 需要修改密码
				user.setPassword(PasswordUtils.encrypt(user.getPassword(), sysUser.getSalt()));
				user.setUpdatedTime(LocalDateTime.now());
				this.userMapper.updateById(user);
			}
		}
	}

	@Override
	public void deleteUser(DeleteUserParam param) {
		SysUser sysUser = this.userMapper.selectById(param.getId());
		if (sysUser.getBeSuper()) {
			throw new CarlosException("不能删除管理员账户");
		}
		this.userMapper.deleteById(param.getId());
	}

	@Override
	public List<SysUser> list() {
		return this.userMapper.selectList(new LambdaQueryWrapper<>(SysUser.class));
	}

	@Override
	public void selective() {
		SysUser sysUser = new SysUser();
		sysUser.setUsername("test2");
		this.userMapper.update(sysUser, new LambdaQueryWrapper<>(SysUser.class).eq(SysUser::getId, "1871915531357876227"));
	}

}
