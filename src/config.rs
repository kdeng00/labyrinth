
/*
pub async fn init_config(url: &str, region: String) -> Result<aws_config::SdkConfig, std::io::Error> {
    let r = region.as_str();
    let config = aws_config::from_env().endpoint_url(url).region(r).load().await;

    Ok(config)
}
*/

pub struct Config {
    pub url: String,
    pub bucket: String,
    pub region: String,
}
