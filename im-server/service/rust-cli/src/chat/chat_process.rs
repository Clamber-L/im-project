use anyhow::Result;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Password};
use reqwest::ClientBuilder;

pub async fn chat_process() -> Result<()> {
    let theme = ColorfulTheme::default();
    let username = Input::<String>::with_theme(&theme)
        .with_prompt("用户名:")
        .interact_text()?;
    let password = Password::with_theme(&theme)
        .with_prompt("密码:")
        .interact()?;
    let client = ClientBuilder::new().gzip(true).build()?;
    let x = client.get("http://localhost:9001/login").send().await?;
    if x.status().is_success() {
        let string = x.text().await?;
        println!("{}", string);
    }

    println!("{}", username);
    println!("{}", password);
    Ok(())
}
