pub mod config;


// use aws_sdk_s3::config::*;

#[derive(Default)]
pub struct Data {
    pub filepath: String,
    pub raw_data: Vec<u8>
}


pub struct Labyrinth {
    pub config: config::Config,
}

pub enum Error {
    Info(String),
    SError(aws_sdk_s3::operation::put_object::PutObjectError)
}

impl Labyrinth {
    pub async fn upload(&self, file_key: &str, data: &Data) -> Result<aws_sdk_s3::operation::put_object::PutObjectOutput, Error> {
        // let config = aws_config::from_env().endpoint_url(&self.config.url).region("").load().await;
        let config = aws_config::load_from_env().await;
        let client = aws_sdk_s3::Client::new(&config);

        let data_content = if data.raw_data.is_empty() {
            match tokio::fs::read(&data.filepath).await {
                Ok(content) => content,
                Err(err) => {
                    eprintln!("Error: {err:?}");
                    Vec::new()
                }
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
            .await {
            Ok(response) => {
                println!("Response: {response:?}");
                println!("Uploaded");
                
                Ok(response)
            }
            Err(err) => {
                eprintln!("Error: {err:?}");
                // aws_sdk_s3::error::SdkError<aws_sdk_s3::operation::put_object::PutObjectError, >
                Err(Error::Info(err.to_string()))
            }
        }
    }
}


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
