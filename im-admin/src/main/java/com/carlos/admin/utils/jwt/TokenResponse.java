package com.carlos.admin.utils.jwt;

import lombok.Getter;
import lombok.Setter;

@Setter
@Getter
public class TokenResponse {

	private String token;

	private String expire;

	public TokenResponse(String token, String expire) {
		this.token = token;
		this.expire = expire;
	}
}
