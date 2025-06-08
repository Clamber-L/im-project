package com.carlos.admin.api.user.vo.param;

import jakarta.validation.constraints.NotNull;
import lombok.Getter;
import lombok.Setter;

@Setter
@Getter
public class DeleteUserParam {

	@NotNull(message = "id不能为空")
	private String id;
}
