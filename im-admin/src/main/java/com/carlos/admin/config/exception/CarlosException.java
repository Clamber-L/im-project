package com.carlos.admin.config.exception;

import lombok.Getter;
import lombok.Setter;
import lombok.extern.slf4j.Slf4j;

import java.text.MessageFormat;

@Slf4j
@Setter
@Getter
public class CarlosException extends RuntimeException {


    private String msg;
    private int code = 500;

    public CarlosException(String msg, Object... args) {
        this.msg = MessageFormat.format(msg, args);
    }

    public CarlosException() {
        this.code = 555;
        this.msg = "系统内部错误";
    }

    public CarlosException(String msg) {
        this.code = 555;
        this.msg = msg;
    }

    public CarlosException(int code, String msg) {
        this.code = code;
        this.msg = msg;
    }

    public CarlosException(String msg, Throwable e) {
        super(msg, e);
        this.msg = msg;
    }

    public CarlosException(int code, String msg, Object... args) {
        this.msg = MessageFormat.format(msg, args);
        this.code = code;
        log.debug(msg);
    }

    public CarlosException(String msg, int code, Throwable e) {
        super(msg, e);
        this.msg = msg;
        this.code = code;
    }
}
