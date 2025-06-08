package com.carlos.admin.config.exception;

import com.carlos.admin.utils.HttpResult;
import lombok.extern.slf4j.Slf4j;
import org.springframework.http.converter.HttpMessageNotReadableException;
import org.springframework.web.bind.annotation.ControllerAdvice;
import org.springframework.web.bind.annotation.ExceptionHandler;
import org.springframework.web.bind.annotation.ResponseBody;

@Slf4j
@ControllerAdvice
public class ExceptionAspect {

    @ResponseBody
    @ExceptionHandler(value = CarlosException.class)
    public Object javaNatExceptionHandler(CarlosException ex) {
        log.debug("javaNatExceptionHandler:{}", ex.getMsg());
        return HttpResult.error(ex.getCode(), ex.getMsg());
    }

    @ResponseBody
    @ExceptionHandler(value = NullPointerException.class)
    public Object javaNpeHandler(NullPointerException ex) {
        log.error("javaExceptionHandler:", ex);
        return HttpResult.error(555, "系统开小差了~");
    }

    @ResponseBody
    @ExceptionHandler(value = Exception.class)
    public Object javaExceptionHandler(Exception ex) {
        log.error("javaExceptionHandler:{}", ex.getLocalizedMessage());
        return HttpResult.error(555, "系统开小差了~");
    }

    @ResponseBody
    @ExceptionHandler(value = HttpMessageNotReadableException.class)
    public Object javaBodyMissExceptionHandler(HttpMessageNotReadableException ex) {
        log.error("javaBodyMissExceptionHandler:{}", ex.getLocalizedMessage());
        return HttpResult.error(555, "参数错误");
    }
}
