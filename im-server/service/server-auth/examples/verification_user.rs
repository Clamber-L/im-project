use lib_core::{generate_jwt, JwtUser};
#[tokio::main]
async fn main() {
    let user = JwtUser {
        id: "1564864212144".to_owned(),
    };

    let token = generate_jwt(user);

    println!("token:{:?}", token);
}
