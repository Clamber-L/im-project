package com.carlos.admin.utils.page;

import cn.hutool.core.convert.Convert;
import com.github.pagehelper.PageHelper;
import jakarta.servlet.http.HttpServletRequest;
import org.springframework.web.context.request.RequestAttributes;
import org.springframework.web.context.request.RequestContextHolder;
import org.springframework.web.context.request.ServletRequestAttributes;

public abstract class PageRequest {

	public static final String PAGE = "page";

	public static final String LIMIT = "limit";

	public static void startPage() {
		int page = getParameterToInt(PAGE, 1);
		int limit = getParameterToInt(LIMIT, 10);
		PageHelper.startPage(page, limit);
	}

	private static Integer getParameterToInt(String name, Integer defaultValue) {
		return Convert.toInt(getRequest().getParameter(name), defaultValue);
	}

	private static HttpServletRequest getRequest() {
		return getRequestAttributes().getRequest();
	}

	private static ServletRequestAttributes getRequestAttributes() {
		RequestAttributes attributes = RequestContextHolder.getRequestAttributes();
		return (ServletRequestAttributes) attributes;
	}
}
