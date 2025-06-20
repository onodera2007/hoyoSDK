use serde::Deserialize;

const DEFAULT: &str = include_str!("../sdk.default.toml");

#[derive(Deserialize)]
pub struct SdkConfig {
    pub http_addr: String,
    pub db_file: String,
}

pub fn load_or_create(path: &str) -> SdkConfig {
    std::fs::read_to_string(path).map_or_else(
        |_| {
            std::fs::write(path, DEFAULT).unwrap();
            toml::from_str(DEFAULT).unwrap()
        },
        |data| toml::from_str(&data).unwrap(),
    )
}
