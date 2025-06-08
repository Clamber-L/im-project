use std::fmt::{Display, Formatter};
use std::path::Path;
use std::str::FromStr;
use anyhow::anyhow;
use clap::Parser;

#[derive(Debug,Parser)]
pub struct CsvOptions {

    #[arg(short,long,value_parser = verify_input_file)]
    pub input: String,

    #[arg(short,long)]
    pub output: Option<String>,

    #[arg(short,long,default_value = "json", value_parser = verify_format)]
    pub format: Format,

    #[arg(short,long,default_value_t = ',')]
    delimiter: char,

    #[arg(long,default_value_t = true)]
    header: bool
}

fn verify_format(format: &str) -> Result<Format,anyhow::Error>{
    format.parse()
}

#[derive(Debug,Copy, Clone)]
pub enum Format {
    Json,
    Toml,
    Yaml,
}

impl From<Format> for &str {
    fn from(value: Format) -> Self {
        match value {
            Format::Json => "json",
            Format::Toml => "toml",
            Format::Yaml => "yaml"
        }
    }
}

impl Display for Format {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl FromStr for Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(Format::Json),
            "toml" => Ok(Format::Toml),
            "yaml" => Ok(Format::Yaml),
            _ => Err(anyhow!("不支持此类型"))
        }
    }
}

fn verify_input_file(file: &str) -> anyhow::Result<String, &'static str> {
    if Path::new(file).exists() {
        Ok(file.into())
    }else {
        Err("文件不存在")
    }
}