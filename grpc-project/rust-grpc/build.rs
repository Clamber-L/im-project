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
                "../proto/user.proto",
                // "../proto/message.proto",
            ],
            &["../proto"],
        )?;
    Ok(())
}
