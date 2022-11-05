use std::{fs::File, io::Read};

use once_cell::sync::OnceCell;
use serde::Deserialize;
use tracing::info;

#[derive(Deserialize, Debug)]
pub struct Conf {
    pub ip: String,
    pub port: u16,
}

static G_CONF: OnceCell<Conf> = OnceCell::new();

impl Conf {
    pub fn global() -> &'static Conf {
        G_CONF.get().expect("配置未初始化")
    }

    pub fn init_conf() {
        let mut str_val = String::new();
        let config_str = match File::open("config.toml") {
            Ok(mut file) => {
                file.read_to_string(&mut str_val).expect("配置文件格式或编码错误");
                str_val.as_str()
            }
            _ => {
                let default_config_str = include_str!("config.toml");
                info!("配置文件不存在, 将使用默认配置: \n {:?}", default_config_str);
                default_config_str
            }
        };
        let conf: Conf = toml::from_str(config_str).expect("解析失败, 请查看你的配置文件");
        G_CONF.set(conf).expect("设置全局配置失败");
    }
}