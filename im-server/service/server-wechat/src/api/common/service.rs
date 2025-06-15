use crate::core::AppState;
use aliyun_oss_rust_sdk::oss::OSS;
use aliyun_oss_rust_sdk::request::RequestBuilder;
use axum::extract::{Multipart, State};
use lib_core::ApiResult;
use lib_utils::{error_result, ok_result};

pub async fn upload(State(state): State<AppState>, mut multipart: Multipart) -> ApiResult<String> {
    println!("upload:{:?}", multipart);
    while let Some(mut field) = multipart.next_field().await? {
        let original_file_name = field.file_name().unwrap_or("unknown_file").to_string();
        let data = field.bytes().await?;

        println!(
            "Length of `{}` is {} bytes, ",
            original_file_name,
            data.len(),
        );

        let oss = OSS::new(
            &state.oss_config.key,
            &state.oss_config.secret,
            &state.oss_config.end_point,
            &state.oss_config.bucket,
        );
        let builder = RequestBuilder::new();
        oss.pub_object_from_buffer(
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
