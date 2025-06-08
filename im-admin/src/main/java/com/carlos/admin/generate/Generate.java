package com.carlos.admin.generate;

import com.baomidou.mybatisplus.generator.FastAutoGenerator;
import com.baomidou.mybatisplus.generator.engine.FreemarkerTemplateEngine;

public class Generate {

	public static void main(String[] args) {
		FastAutoGenerator.create("jdbc:mysql://47.95.179.146:3306/im-project?serverTimezone=GMT%2B8", "root", "Lsw@0516")
				.globalConfig(builder -> {
					builder.outputDir("src/main/java"); // 输出目录
				})
				.packageConfig(builder -> {
					builder.parent("com.carlos.admin.generate") // 设置父包名
							.entity("domain") // 设置实体类包名
							.mapper("mapper") // 设置 Mapper 接口包名
							.service("service") // 设置 Service 接口包名
							.serviceImpl("service.impl") // 设置 Service 实现类包名
							.xml("mappers"); // 设置 Mapper XML 文件包名
				})
				.strategyConfig(builder -> {
					builder.addInclude("sys_permission", "sys_user_role","sys_role_permission") // 设置需要生成的表名
							.entityBuilder()
							.enableLombok() // 启用 Lombok
							.enableTableFieldAnnotation() // 启用字段注解
							.controllerBuilder()
							.enableRestStyle(); // 启用 REST 风格
				})
				.templateEngine(new FreemarkerTemplateEngine()) // 使用 Freemarker 模板引擎
				.execute(); // 执行生成
	}
}
