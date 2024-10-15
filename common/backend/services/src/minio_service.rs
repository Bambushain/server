use bytes::Bytes;
use s3::creds::Credentials;
use s3::error::S3Error;
use s3::{Bucket, Region};
use std::str::FromStr;

use bamboo_common_core::error::{BambooError, BambooErrorResult, BambooResult};

#[derive(Clone)]
pub struct MinioClient {
    bucket: Box<Bucket>,
}

impl MinioClient {
    pub fn new() -> Result<MinioClient, S3Error> {
        let bucket_name =
            std::env::var("S3_BUCKET").map_err(|err| S3Error::Io(std::io::Error::other(err)))?;
        let access_key = std::env::var("S3_ACCESS_KEY")
            .map_err(|err| S3Error::Io(std::io::Error::other(err)))?;
        let secret_key = std::env::var("S3_SECRET_KEY")
            .map_err(|err| S3Error::Io(std::io::Error::other(err)))?;
        let region =
            std::env::var("S3_REGION").map_err(|err| S3Error::Io(std::io::Error::other(err)))?;
        let endpoint = std::env::var("S3_ENDPOINT").ok();
        let use_path_style = std::env::var("S3_USE_PATH_STYLE")
            .ok()
            .map_or(false, |val| val.to_lowercase() == "true");

        let region = if let Some(endpoint) = endpoint {
            Region::Custom { region, endpoint }
        } else {
            Region::from_str(region.as_str())?
        };
        let credentials = Credentials::new(
            Some(access_key.as_str()),
            Some(secret_key.as_str()),
            None,
            None,
            None,
        )
        .map_err(S3Error::Credentials)?;
        let mut bucket = Bucket::new(bucket_name.as_str(), region, credentials)?.with_path_style();
        if use_path_style {
            bucket.set_path_style();
        }

        Ok(MinioClient { bucket })
    }

    fn get_profile_picture_path(&self, user_id: i32) -> String {
        format!("/user/profile_picture/{user_id}")
    }

    pub async fn upload_profile_picture(&self, user_id: i32, data: &[u8]) -> BambooErrorResult {
        let response = self
            .bucket
            .put_object(self.get_profile_picture_path(user_id), data)
            .await
            .map_err(|_| BambooError::io("user", "Failed to save profile picture"))?;
        if response.status_code() != 200 {
            Err(BambooError::io("user", "Failed to save profile picture"))
        } else {
            Ok(())
        }
    }

    pub async fn get_profile_picture(&self, user_id: i32) -> BambooResult<Bytes> {
        let response = self
            .bucket
            .get_object(self.get_profile_picture_path(user_id))
            .await
            .map_err(|_| BambooError::io("user", "Failed to get profile picture"))?;
        if response.status_code() != 200 {
            Err(BambooError::io("user", "Failed to get profile picture"))
        } else {
            let data = response.bytes().clone();
            Ok(data)
        }
    }
}
