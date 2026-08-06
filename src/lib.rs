pub mod config;

use aws_sdk_s3::config::{Credentials, Region};

#[derive(Default)]
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
        // 1. Create credentials
        let credentials = Credentials::new(
            self.config.access_key_id.clone(),
            self.config.secret_key.clone(),
            None,   // session_token - None for long-term credentials
            None,   // expires_after - None for long-term credentials
            "maze", // provider_name - just a label
        );

        // 2. Create region
        let region = Region::new(self.config.region.to_string());
        let config = aws_config::from_env()
            .region(region)
            .credentials_provider(credentials)
            .endpoint_url(self.config.url.clone())
            .load()
            .await;

        // let config = aws_config::load_from_env().await;
        let client = aws_sdk_s3::Client::new(&config);

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
