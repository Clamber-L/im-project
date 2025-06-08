package com.carlos.admin.utils;

import lombok.extern.slf4j.Slf4j;

import javax.crypto.Cipher;
import javax.crypto.SecretKey;
import javax.crypto.SecretKeyFactory;
import javax.crypto.spec.PBEKeySpec;
import javax.crypto.spec.PBEParameterSpec;
import java.nio.charset.StandardCharsets;
import java.security.Key;
import java.util.Random;

@Slf4j
public class PasswordUtils {

	/**
	 * 定义使用的算法为:PBEWITHMD5andDES算法
	 * 加密算法
	 */
	public static final String ALGORITHM = "PBEWithMD5AndDES";

	/**
	 * 定义迭代次数为1000次
	 */
	private static final int IterationCount = 1000;

	public static String randomGen(int place) {
		String base = "qwertyuioplkjhgfdsazxcvbnmQAZWSXEDCRFVTGBYHNUJMIKLOP0123456789";
		StringBuilder sb = new StringBuilder();
		Random rd = new Random();
		for(int i=0;i<place;i++) {
			sb.append(base.charAt(rd.nextInt(base.length())));
		}
		return sb.toString();
	}

	/**
	 * 加密明文字符串
	 * @param password
	 *            生成密钥时所使用的密码
	 * @param salt
	 *            盐值
	 * @return 加密后的密文字符串
	 */
	public static String encrypt(String password, String salt) {
		Key key = getPbeKey(password);
		byte[] encipheredData = null;
		PBEParameterSpec parameterSpec = new PBEParameterSpec(salt.getBytes(), IterationCount);
		try {
			Cipher cipher = Cipher.getInstance(ALGORITHM);

			cipher.init(Cipher.ENCRYPT_MODE, key, parameterSpec);
			encipheredData = cipher.doFinal(password.getBytes(StandardCharsets.UTF_8));
		} catch (Exception e) {
			log.error("encrypt password error");
		}
		return bytesToHexString(encipheredData);
	}

	/**
	 * 将字节数组转换为十六进制字符串
	 *
	 * @param src
	 *            字节数组
	 * @return
	 */
	private static String bytesToHexString(byte[] src) {
		StringBuilder stringBuilder = new StringBuilder();
		if (src == null || src.length == 0) {
			return null;
		}
		for (byte b : src) {
			int v = b & 0xFF;
			String hv = Integer.toHexString(v);
			if (hv.length() < 2) {
				stringBuilder.append(0);
			}
			stringBuilder.append(hv);
		}
		return stringBuilder.toString();
	}

	/**
	 * 根据PBE密码生成一把密钥
	 *
	 * @param password
	 *            生成密钥时所使用的密码
	 * @return Key PBE算法密钥
	 * */
	private static Key getPbeKey(String password) {
		// 实例化使用的算法
		SecretKeyFactory keyFactory;
		SecretKey secretKey = null;
		try {
			keyFactory = SecretKeyFactory.getInstance(ALGORITHM);
			// 设置PBE密钥参数
			PBEKeySpec keySpec = new PBEKeySpec(password.toCharArray());
			// 生成密钥
			secretKey = keyFactory.generateSecret(keySpec);
		} catch (Exception e) {
			log.error("get pb key error");
		}

		return secretKey;
	}
}
