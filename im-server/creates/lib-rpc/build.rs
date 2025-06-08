use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    fs::create_dir_all("src/pb")?;
    let builder = tonic_build::configure();

    builder
        .out_dir("src/pb")
        .type_attribute(
            ".",
            "#[derive(serde::Serialize,serde::Deserialize)] #[serde(rename_all = \"camelCase\")]",
        )
        .compile_protos(
            &[
                "../../protos/user/user.proto",
                "../../protos/message/message.proto",
            ],
            &["../../protos"],
        )?;

    // builder
    //     .out_dir("src/pb")
    //     .with_serde(
    //         &["User", "Message"],
    //         true,
    //         true,
    //         Some(&[r#"#[serde(rename_all = "camelCase")]"#]),
    //     )
    //     .with_derive_builder(&["User", "Message"], None)
    //     .compile_protos(
    //         &[
    //             "../../protos/user/user.proto",
    //             "../../protos/message/message.proto",
    //         ],
    //         &["../../protos"],
    //     )?;

    Ok(())
}
