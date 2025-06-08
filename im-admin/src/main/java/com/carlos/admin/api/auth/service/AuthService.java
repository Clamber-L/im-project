package com.carlos.admin.api.auth.service;

import com.carlos.admin.api.auth.vo.LoginParam;
import com.carlos.admin.utils.jwt.TokenResponse;

public interface AuthService {

	/**
	 * 登录
	 *
	 * @param param
	 * @return
	 */
	TokenResponse login(LoginParam param);

	void currentUser();

}
