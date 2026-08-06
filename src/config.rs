#[derive(Debug, Default)]
pub struct Config {
    pub url: String,
    pub bucket: String,
    pub region: String,
    pub access_key_id: String,
    pub secret_key: String,
}
