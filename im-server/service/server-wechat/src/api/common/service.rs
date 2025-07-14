use crate::core::AppState;
use aliyun_oss_rust_sdk::request::RequestBuilder;
use axum::extract::{Multipart, State};
use lib_core::ApiResult;
use lib_utils::{error_result, ok_result};
use tracing::info;

pub async fn upload(State(state): State<AppState>, mut multipart: Multipart) -> ApiResult<String> {
    println!("upload:{:?}", multipart);
    while let Some(field) = multipart.next_field().await? {
        let original_file_name = field.file_name().unwrap_or("unknown_file").to_string();
        let data = field.bytes().await?;

        info!(
            "Length of `{}` is {} bytes, ",
            original_file_name,
            data.len(),
        );

        let builder = RequestBuilder::new();
        &state
            .oss
            .pub_object_from_buffer(
                format!("applet/{}", original_file_name),
                data.as_ref(),
                builder,
            )
            .await?;
        let full_path_name = format!(
            "https://println-g1-carlos.oss-cn-qingdao.aliyuncs.com/applet/{}",
            original_file_name
        );
        return Ok(ok_result(full_path_name));
    }
    Ok(error_result("上传失败"))
}
