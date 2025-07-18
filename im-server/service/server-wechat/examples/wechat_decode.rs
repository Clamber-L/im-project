use wechat_pay_rust_sdk::pay::{PayNotifyTrait, WechatPay};

fn main() {
    // 初始化支付
    let wechat_pay = WechatPay::from_env();

    let ciphertext = "";

    let associated_data = "transaction";

    let nonce = "";

    let data = wechat_pay
        .decrypt_paydata(ciphertext, nonce, associated_data)
        .unwrap();
    println!("data: {:#?}", data);
}
