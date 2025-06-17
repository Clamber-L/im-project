use std::fs::read_to_string;
use wechat_pay_rust_sdk::pay::{PayNotifyTrait, WechatPay};

fn main() {
    // 初始化支付
    let contents = read_to_string("./apiclient_key.pem");
    let contents = contents.unwrap();
    let wechat_pay = WechatPay::new(
        "wx89003765f262e336",
        "1719719929",
        &contents,
        "502138A5DE660F670A75300D0B97BA2FD38C2E8D",
        "zm1Pbe4IeBr4GXuwGs0TbxEyfiAQFKmC",
        "https://42cf3dcb.r30.cpolar.top/applet/pay_callback",
    );

    let  ciphertext =
    "taCMoed+P+cRTuUTDRnnUPNjx8WfuZ+QhRY7ruamT7eNwbKRATWPovQVbofhhVX9x7IOhQQXvgc6LUXm483Adt+5NEOnEMFEvM2Qs7XR6Rz+E/Ju9pdn+2bIfQ6cemeF1PLJ6THrYYTYn5mLoeYpEnEaT6lmKQIeILt7HL0czT6cL6OZscLzRleY8i9kjfdbyY9s6wxszIjYHaekT9HKnbOn8sr88VB6bCd+WvlV6YNSwULWZ1fTj2l0JWo5nczQRuUs0VP2GdHbCxqhtiUEHayyoHVm9Y9kcjhO4S6d0uIUsXkgRV9WJ5K0B/YONwmzxn73QDPqyQ6XyK5JZZ6PG6yKg1wcO2WCvkcrfIh/Zm3Nw4u19A4fXlmWVZAN+YlaYZtdcCtuYCDaCpkeJ+4oofEO/XjueUL7rwOagqLzi9FOCGLHDllbIVaEFazsMCgdID3XsR+89fHWMw2ElUjsyF88qJOp0x5Zrnes8evuvPqNX6aOOJZZas8OewX/rvpzL9ppH4CHpFk88f1k6cAwTjgMowqpaE8HwIUE06BxRpX3mkYPxRljsNqDeg==";

    let associated_data = "transaction";

    let nonce = "QKdBXU10kVZD";

    let data = wechat_pay
        .decrypt_paydata(ciphertext, nonce, associated_data)
        .unwrap();
    println!("data: {:#?}", data);
}
