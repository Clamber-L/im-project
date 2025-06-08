package com.carlos.admin.api.auth.vo;

import jakarta.validation.constraints.NotBlank;
import lombok.Getter;
import lombok.Setter;

@Setter
@Getter
public class LoginParam {

	@NotBlank
	private String account;

	@NotBlank
	private String password;
}
