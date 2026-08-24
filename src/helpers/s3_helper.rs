use std::{collections::HashMap, fmt, io::{Cursor, Write}};

use aws_sdk_s3::{error::SdkError, operation::{get_object::GetObjectOutput, head_object::HeadObjectOutput, put_object::PutObjectError}, primitives::{ByteStream, ByteStreamError}, Client};

pub struct S3Object {
    pub key: String,
    pub etag: String,
}

#[derive(Default)]
pub struct PutOptions {
    pub content_type: Option<String>,
    pub cache_control: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
}

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

    /// Lists every key under a prefix.
    pub async fn list_objects(
        &self,
        prefix: &str
    ) -> Vec<String> {
        self.list_objects_with_etags(prefix)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|object| object.key)
            .collect()
    }

    /// Lists every key under a prefix along with its ETag.
    pub async fn list_objects_with_etags(
        &self,
        prefix: &str
    ) -> Result<Vec<S3Object>, S3Error> {
        let mut results: Vec<S3Object> = Vec::new();
        let mut continuation_token = None;

        loop {
            let loo = self.s3_client
                .list_objects_v2()
                .bucket(&self.bucket)
                .prefix(prefix)
                .set_continuation_token(continuation_token)
                .send()
                .await
                .map_err(|err| S3Error{message: err.to_string()})?;

            for object in loo.contents() {
                let key = object.key().map_or("", |key| key);
                if !key.is_empty() && !key.ends_with("/") {
                    results.push(S3Object{
                        key: key.to_string(),
                        // Quoted on the wire; callers compare it to what they stored.
                        etag: object.e_tag().unwrap_or("").trim_matches('"').to_string(),
                    });
                }
            }

            if loo.is_truncated().unwrap_or(false) {
                continuation_token = loo.next_continuation_token().map(str::to_string);
            } else {
                return Ok(results);
            }
        }
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

    pub async fn put_object(
        &self, 
        key: &str, 
        body: Vec<u8>
    ) -> Result<(), SdkError<PutObjectError>> {
        self.put_object_with(key, body, PutOptions::default()).await
    }

    /// Writes an object along with the attributes S3 should serve it with.
    pub async fn put_object_with(
        &self,
        key: &str,
        body: Vec<u8>,
        options: PutOptions
    ) -> Result<(), SdkError<PutObjectError>> {
        self.s3_client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(ByteStream::from(body))
            .set_content_type(options.content_type)
            .set_cache_control(options.cache_control)
            .set_metadata(options.metadata)
            .send()
            .await?;

        return Ok(());
    }    

    pub async fn exists(
        &self,
        key: &str
    ) -> bool {
        return matches!(self.head_object(key).await, Ok(Some(_)));
    }

    /// Reads an object's attributes without its body.
    pub async fn head_object(
        &self,
        key: &str
    ) -> Result<Option<HeadObjectOutput>, S3Error> {
        return match self.s3_client
            .head_object()
            .bucket(&self.bucket)
            .key(key)
            .send().await {
                Ok(head) => Ok(Some(head)),
                Err(err) => {
                    let err_msg = err.to_string();
                    if err.into_service_error().is_not_found() {
                        Ok(None)
                    } else {
                        Err(S3Error{message: err_msg})
                    }
                }
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