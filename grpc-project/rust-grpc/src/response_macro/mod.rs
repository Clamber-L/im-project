#[macro_export]
macro_rules! grpc_try {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => {
                eprintln!("gRPC error: [{}] {}", err.code(), err.message());
                return Ok(());
            }
        }
    };
}
