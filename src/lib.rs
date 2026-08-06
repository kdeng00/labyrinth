pub mod config;

use aws_sdk_s3::config::{Credentials, Region};

#[derive(Debug, Default)]
pub struct Data {
    pub filepath: String,
    pub raw_data: Vec<u8>,
}

pub struct Labyrinth {
    pub config: config::Config,
}

pub async fn load_data(filepath: &str) -> Result<Vec<u8>, std::io::Error> {
    tokio::fs::read(filepath).await
}

pub async fn init_client(config: &config::Config) -> aws_sdk_s3::Client {
    let credentials = Credentials::new(
        config.access_key_id.clone(),
        config.secret_key.clone(),
        None,
        None,
        "maze",
    );

    let region = Region::new(config.region.to_string());
    let config = aws_config::from_env()
        .region(region)
        .credentials_provider(credentials)
        .endpoint_url(config.url.clone())
        .load()
        .await;

    let s3_config = aws_sdk_s3::config::Builder::from(&config)
        .force_path_style(true)
        .build();

    let client = aws_sdk_s3::Client::from_conf(s3_config);
    client
}

#[derive(Debug)]
pub enum Error {
    Info(String),
    SError(aws_sdk_s3::operation::put_object::PutObjectError),
}

impl Labyrinth {
    pub async fn upload(
        &self,
        file_key: &str,
        data: &Data,
    ) -> Result<aws_sdk_s3::operation::put_object::PutObjectOutput, Error> {
        let client = init_client(&self.config).await;
        let data_content = if data.raw_data.is_empty() {
            match load_data(&data.filepath).await {
                Ok(content) => content,
                Err(err) => return Err(Error::Info(err.to_string())),
            }
        } else {
            data.raw_data.to_owned()
        };

        let body = aws_sdk_s3::primitives::SdkBody::from(data_content);
        let b_stream = aws_sdk_s3::primitives::ByteStream::from(body);
        match client
            .put_object()
            .bucket(self.config.bucket.clone())
            .key(file_key)
            .body(b_stream)
            .send()
            .await
        {
            Ok(response) => Ok(response),
            Err(err) => Err(Error::Info(err.to_string())),
        }
    }
}
