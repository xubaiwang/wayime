use figment::{
    providers::{Format, Toml},
    Figment,
};
use im::Im;
use serde::{Deserialize, Deserializer};
use wayland_client::Connection;
use xkbcommon::xkb::{Keysym, KEYSYM_NO_FLAGS};

mod engine;
mod im;

fn main() {
    // 初始化日誌輸出
    env_logger::init();

    // load config
    let config_file = dirs::config_dir()
        .expect("fail to get config dir")
        .join("wlrime")
        .join("config.toml");
    let config: Config = Figment::new()
        .merge(Toml::file(config_file))
        .extract()
        .expect("Fail to load config");
    dbg!(&config);

    // 連接 wayland
    let conn = Connection::connect_to_env().unwrap();
    let mut event_queue = conn.new_event_queue();
    let qh = event_queue.handle();

    // 初始化輸入法
    let mut im = Im::new(config);
    let display = conn.display();
    display.get_registry(&qh, ());

    // 循環
    loop {
        event_queue.blocking_dispatch(&mut im).unwrap();
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    #[serde(
        deserialize_with = "deserialize_keysym_from_name",
        default = "default_switch_key"
    )]
    pub switch_key: Keysym,
}

fn deserialize_keysym_from_name<'de, D>(deserializer: D) -> Result<Keysym, D::Error>
where
    D: Deserializer<'de>,
{
    let name = String::deserialize(deserializer)?;
    Ok(xkbcommon::xkb::keysym_from_name(&name, KEYSYM_NO_FLAGS))
}

fn default_switch_key() -> Keysym {
    Keysym::XF86_Keyboard
}
