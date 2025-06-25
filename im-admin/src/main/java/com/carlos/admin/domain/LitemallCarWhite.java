package com.carlos.admin.domain;

import com.baomidou.mybatisplus.annotation.TableId;
import com.baomidou.mybatisplus.annotation.TableName;
import lombok.Getter;
import lombok.Setter;
import lombok.ToString;

import java.io.Serializable;
import java.time.LocalDateTime;

@Getter
@Setter
@ToString
@TableName("litemall_car_white")
public class LitemallCarWhite implements Serializable {

    @TableId("id")
    private Integer id;

    private String carNum;

    private Boolean deleted;

    private LocalDateTime addTime;

    private String name;

    private String remark;
}