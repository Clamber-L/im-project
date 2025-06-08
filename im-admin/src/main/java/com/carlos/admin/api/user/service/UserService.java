package com.carlos.admin.api.user.service;

import com.carlos.admin.api.user.vo.param.DeleteUserParam;
import com.carlos.admin.domain.SysUser;
import org.springframework.security.core.userdetails.UserDetailsService;

import java.util.List;

public interface UserService extends UserDetailsService {

	/**
	 * 获取用户信息
	 */
	void userInfo();

	/**
	 * 保存用户信息
	 * @param user
	 */
	void saveUser(SysUser user);

	/**
	 * 删除用户
	 * @param param
	 */
	void deleteUser(DeleteUserParam param);

	/**
	 * 用户列表
	 *
	 * @return
	 */
    List<SysUser> list();

	/**
	 * 可选择的
	 */
	void selective();
}
