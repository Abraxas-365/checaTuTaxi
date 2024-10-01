use async_trait::async_trait;

use crate::{
    error::ApiError,
    utils::database::{PaginatedRecord, Pagination},
};

use super::{Complaint, ComplaintImage, Driver, DriverImage, Location};

#[async_trait]
pub trait DBRepository: Send + Sync {
    // Driver operations
    async fn create_driver(&self, driver: &Driver) -> Result<Driver, ApiError>;
    async fn get_driver_by_id(&self, id: i32) -> Result<Driver, ApiError>;
    async fn get_driver_by_license_plate(&self, license_plate: &str) -> Result<Driver, ApiError>;
    async fn update_driver(&self, driver: &Driver) -> Result<Driver, ApiError>;
    async fn delete_driver(&self, id: i32) -> Result<(), ApiError>;
    async fn search_drivers(
        &self,
        query: &str,
        pagination: &Pagination,
    ) -> Result<PaginatedRecord<Driver>, ApiError>;

    // Driver Image operations
    async fn add_driver_image(&self, driver_image: &DriverImage) -> Result<DriverImage, ApiError>;
    async fn get_driver_images(
        &self,
        driver_id: i32,
        pagination: &Pagination,
    ) -> Result<PaginatedRecord<DriverImage>, ApiError>;
    async fn delete_driver_image(&self, id: i32) -> Result<(), ApiError>;

    // Complaint operations
    async fn create_complaint(&self, complaint: &Complaint) -> Result<Complaint, ApiError>;
    async fn get_complaint_by_id(&self, id: i32) -> Result<Complaint, ApiError>;
    async fn update_complaint(&self, complaint: &Complaint) -> Result<Complaint, ApiError>;
    async fn delete_complaint(&self, id: i32) -> Result<(), ApiError>;
    async fn get_complaints_for_driver(
        &self,
        driver_id: i32,
        pagination: &Pagination,
    ) -> Result<PaginatedRecord<Complaint>, ApiError>;

    // Complaint Image operations
    async fn add_complaint_image(
        &self,
        complaint_image: &ComplaintImage,
    ) -> Result<ComplaintImage, ApiError>;
    async fn get_complaint_images(
        &self,
        complaint_id: i32,
        pagination: &Pagination,
    ) -> Result<PaginatedRecord<ComplaintImage>, ApiError>;
    async fn delete_complaint_image(&self, id: i32) -> Result<(), ApiError>;

    // Location operations
    async fn get_location_by_id(&self, id: i32) -> Result<Location, ApiError>;
    async fn get_locations_by_country(
        &self,
        country: &str,
        pagination: &Pagination,
    ) -> Result<PaginatedRecord<Location>, ApiError>;
    async fn get_all_locations(
        &self,
        pagination: &Pagination,
    ) -> Result<PaginatedRecord<Location>, ApiError>;
}

#[async_trait]
pub trait BucketPort: Send + Sync {
    /// Generate a presigned URL for uploading an image to the bucket
    /// Returns the presigned URL and the public URL where the image will be accessible after upload
    async fn generate_upload_url(&self, file_name: &str) -> Result<(String, String), ApiError>;

    /// Delete an image from the bucket
    async fn delete_image(&self, file_name: &str) -> Result<(), ApiError>;
}
