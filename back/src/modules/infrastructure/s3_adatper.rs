use async_trait::async_trait;
use std::time::Duration;

use crate::error::ApiError;
use crate::modules::port::BucketPort;
use crate::utils::s3::S3Repository;

#[async_trait]
impl BucketPort for S3Repository {
    async fn generate_upload_url(&self, key: &str) -> Result<(String, String), ApiError> {
        let presigned_url = self
            .post_presigned_url(key, Duration::from_secs(3600))
            .await?;

        let public_url = format!("https://{}.s3.amazonaws.com/{}", self.bucket, key);

        Ok((presigned_url, public_url))
    }

    async fn delete_image(&self, key: &str) -> Result<(), ApiError> {
        self.delete_object(key).await
    }
}
