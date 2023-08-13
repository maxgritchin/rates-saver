use my_seq_logger::SeqSettings;
use yh_services_sdk::yh_config::Config;

use super::AppConfig;

const CONFIG_FILE: &'static str = ".rates-saver";
const LOCAL_CONFIG: &'static str = ".config.yml";

#[async_trait::async_trait]
impl SeqSettings for AppConfig {
    async fn get_conn_string(&self) -> String {
        self.seq_url.to_string()
    }
}

pub fn load() -> AppConfig {
    Config::new()
        .set_yaml_file_name(CONFIG_FILE)
        .set_yaml_file_name(LOCAL_CONFIG)
        .read_settings_from_url_via_env_var()
        .add_env()
        .read::<AppConfig>()
}
