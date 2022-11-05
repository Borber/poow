use std::{fs::File, io::Read};

use once_cell::sync::Lazy;
use serde::Deserialize;
use tracing::info;

#[derive(Deserialize, Debug)]
pub struct Conf {
    pub ip: String,
    pub port: u16,
}

static DEFAULT_CONFIG_LOCATION: &str = "config.toml";
static DEFAULT_CONFIG: &str = include_str!("config.toml");

pub static G_CONF: Lazy<Conf> = Lazy::new(|| {
    let mut str_val = String::new();
    let config_str = match File::open(DEFAULT_CONFIG_LOCATION) {
        Ok(mut file) => {
            file.read_to_string(&mut str_val)
                .expect("配置文件格式或编码错误");
            str_val.as_str()
        }
        _ => {
            info!("配置文件不存在, 将使用默认配置: \n{}", DEFAULT_CONFIG);
            DEFAULT_CONFIG
        }
    };
    let conf: Conf = toml::from_str(config_str).expect("解析失败, 请查看你的配置文件");
    conf
});
