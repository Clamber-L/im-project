package com.carlos.admin.utils.jwt;

import com.auth0.jwt.JWT;
import com.auth0.jwt.JWTVerifier;
import com.auth0.jwt.algorithms.Algorithm;
import com.auth0.jwt.exceptions.JWTCreationException;
import com.auth0.jwt.exceptions.JWTVerificationException;
import com.auth0.jwt.interfaces.Claim;
import com.auth0.jwt.interfaces.DecodedJWT;
import com.carlos.admin.config.exception.CarlosException;
import lombok.extern.slf4j.Slf4j;

import java.time.Instant;
import java.time.LocalDateTime;
import java.time.ZoneOffset;
import java.util.*;

@Slf4j
public class JwtUtils {
    // 秘钥
    static final String SECRET = "dlkfh6544567agsldf34534h23423iwefdf563654";
    // 签名是有谁生成
    static final String ISSUSER = "Carlos";
    // 签名的主题
    static final String SUBJECT = "TRY_AGAIN";
    // 签名的观众
    static final String AUDIENCE = "KIKO";


    public static TokenResponse createToken(String userId) {
        try {
            Algorithm algorithm = Algorithm.HMAC256(SECRET);
            Map<String, Object> map = new HashMap<String, Object>();
            // 过期时间：2小时
            LocalDateTime expireDate = LocalDateTime.now().plusHours(5L);

            Instant instant = expireDate.atOffset(ZoneOffset.ofHours(8)).toInstant();

            map.put("alg", "HS256");
            map.put("typ", "JWT");
            String token = JWT.create()
                    // 设置头部信息 Header
                    .withHeader(map)
                    // 设置 载荷 Payload
                    .withClaim("userId", userId)
                    .withClaim("expire", Date.from(instant))
                    .withClaim("version", "1")
                    .withIssuer(ISSUSER)
                    .withSubject(SUBJECT)
                    .withAudience(AUDIENCE)
                    // 生成签名的时间
                    .withIssuedAt(new Date())
                    // 签名过期的时间
                    .withExpiresAt(Date.from(instant))
                    // 签名 Signature
                    .sign(algorithm);
            return new TokenResponse(token, Long.toString(instant.getEpochSecond()));
        } catch (JWTCreationException exception) {
            log.error("create token error:{}", exception.getMessage());
        }
        return null;
    }

    public static UserBean verifyTokenAndGetUserBean(String token) {
        try {
            Algorithm algorithm = Algorithm.HMAC256(SECRET);
            JWTVerifier verifier = JWT.require(algorithm)
                    .withIssuer(ISSUSER)
                    .build();
            DecodedJWT jwt = verifier.verify(token);
            Map<String, Claim> claims = jwt.getClaims();

            Claim userIdClaim = claims.get("userId");
            Claim expire = claims.get("expire");
            String userId = userIdClaim.asString();
            String expireDate  = expire == null ? null : expire.asString();

            return new UserBean(userId,expireDate);
        } catch (JWTVerificationException exception) {
            log.error("jwt解密失败  token：{}",token);
            throw new CarlosException("登录状态失效，请重新登录");
        }
    }
}
