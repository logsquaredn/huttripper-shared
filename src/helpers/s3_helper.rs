use std::{fmt, io::{Cursor, Write}};

use aws_sdk_s3::{operation::{get_object::GetObjectOutput, list_objects_v2::ListObjectsV2Output}, primitives::ByteStreamError, types::Object, Client};

#[derive(Clone)]
pub struct S3Helper {
    pub s3_client: Client,
    pub bucket: String,
}

pub fn create_s3_helper(aws_config: &aws_config::SdkConfig, bucket: &str) -> S3Helper {
    let s3_client: aws_sdk_s3::Client = aws_sdk_s3::Client::new(aws_config);
    S3Helper {
        s3_client,
        bucket: bucket.to_string()
    }
}

impl S3Helper {

    pub async fn list_objects(
        &self,
        prefix: &str
    ) -> Vec<String> {
        let mut results: Vec<String> = Vec::new();
        let loo: ListObjectsV2Output = match self.s3_client
            .list_objects_v2()
            .bucket(&self.bucket)
            .prefix(prefix)
            .send()
            .await {
                Err(_) => {
                    return results;
                },
                Ok(res) => res 
            };
        let objects: &[Object] = loo
            .contents();

        for object in objects {
            let key = object.key().map_or("", |key| key);
            if !key.is_empty() && !key.ends_with("/") {
                results.push(key.to_string());
            }
        }

        return results;
    }   

    pub async fn get_object(&self, key: &str) -> Result<Option<GetObjectOutput>, S3Error> {
        let maybe_object: Option<GetObjectOutput> = match self.s3_client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await {
                Ok(object) => {
                    Ok(Some(object))
                }
                Err(err) => {
                    let err_msg = err.to_string();
                    if err.into_service_error().is_no_such_key() {
                        Ok(None)
                    } else {
                        Err(S3Error{message: err_msg})
                    }
                }
            }?;

        Ok(maybe_object)
    }

    pub async fn read_object_bytes(&self, mut object: GetObjectOutput) -> Result<Cursor<Vec<u8>>, ByteStreamError> {
        let mut mem = Cursor::new(Vec::new());
        while let Some(bytes) = object.body.try_next().await?  {
            mem.write_all(&bytes)?;
        }

        Ok(mem)
    }

    pub async fn exists(
        &self,
        key: &str
    ) -> bool {
        return match self.s3_client
            .head_object()
            .bucket(&self.bucket)
            .key(key)
            .send().await {
                Err(_) => false,
                Ok(_) => true
        };
    }
}

#[derive(Debug)]
pub struct S3Error {
    pub message: String
}

impl fmt::Display for S3Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for S3Error {
    fn description(&self) -> &str {
        &self.message
    }
}