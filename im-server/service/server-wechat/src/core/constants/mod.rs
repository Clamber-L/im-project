/// 获取access_token
pub const ACCESS_TOKEN_URL: &'static str =
    "https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential";

/// 微信小程序登录
pub const LOGIN_URL: &'static str = "https://api.weixin.qq.com/sns/jscode2session";

/// 获取登录用户的手机号码
pub const LOGIN_USER_PHONE_URL: &'static str =
    "https://api.weixin.qq.com/wxa/business/getuserphonenumber";

/// 微信支付接口api
pub const WECHAT_PAY_API: &'static str = "https://api.mch.weixin.qq.com/v3/pay/transactions/jsapi";
