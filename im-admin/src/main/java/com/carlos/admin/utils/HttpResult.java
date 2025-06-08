package com.carlos.admin.utils;

import lombok.Getter;
import lombok.Setter;
import org.springframework.http.HttpStatus;

import java.util.HashMap;
import java.util.Map;

@Setter
@Getter
public class HttpResult<T> {

    private int code = 200;
    private String msg;
    private T data;

    public static <T> HttpResult<T> error() {
        return error(HttpStatus.INTERNAL_SERVER_ERROR.value(), "未知异常，请联系管理员");
    }

    public static <T> HttpResult<T> error(String msg) {
        return error(506, msg);
    }

    public static <T> HttpResult<T> error_500(String msg) {
        return error(500, msg);
    }

    public static <T> HttpResult<T> error(int code, String msg) {
        HttpResult<T> r = new HttpResult<>();
        r.setCode(code);
        r.setMsg(msg);
        return r;
    }

    public static <T> HttpResult<T> error(int code, String msg, String type) {
        HttpResult<T> r = new HttpResult<>();
        r.setCode(code);
        r.setMsg(msg);
        return r;
    }


    public static <T> HttpResult<T> errorData(int code, String msg) {
        return error(code, msg);
    }


    public static <T> HttpResult<T> errorData(int code, String msg, T data) {
        HttpResult<T> r = new HttpResult<>();
        r.setCode(code);
        r.setMsg(msg);
        r.setData(data);
        return r;
    }

    public static <T> HttpResult<T> errorNoAuth(String msg) {
        HttpResult<T> r = new HttpResult<>();
        r.setCode(401);
        r.setMsg(msg);
        return r;
    }


    public static <T> HttpResult<T> okWithMsg(String msg) {
        HttpResult<T> r = new HttpResult<>();
        r.setMsg(msg);
        return r;
    }

    public static <T> HttpResult<T> ok(String msg, T obj) {
        HttpResult<T> r = new HttpResult<>();
        r.setMsg(msg);
        r.setData(obj);
        return r;
    }

    public static <T> HttpResult<T> ok(T data) {
        HttpResult<T> r = new HttpResult<>();
        r.setData(data);
        return r;
    }


    public static HttpResult<Map<String, Object>> okMap(Object data) {
        Map<String, Object> resultMap = new HashMap<String, Object>();
        resultMap.put("obj", data);
        HttpResult<Map<String, Object>> r = new HttpResult<>();
        r.setData(resultMap);
        return r;
    }

    public static <T> HttpResult<T> ok() {
        return new HttpResult<>();
    }

	public static <T> HttpResult<T> win(Integer retCode, String msg, T retData) {
        HttpResult<T> r = new HttpResult<T>();
        r.code = retCode;
        r.msg = msg;
        r.data = retData;
        return r;
    }

    @Override
    public String toString() {
        final StringBuffer sb = new StringBuffer("Res{");
        sb.append("code=").append(code);
        sb.append(", msg='").append(msg).append('\'');
        sb.append(", data=").append(data);
        sb.append('}');
        return sb.toString();
    }

}
