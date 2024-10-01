use super::{
    port::{BucketPort, DBRepository},
    Complaint, ComplaintImage, ComplaintWithImages, Driver, DriverImage, DriverWithDetails,
    DriverWithImages, NewComplaint,
};
use crate::{
    error::ApiError,
    utils::database::{PaginatedRecord, Pagination},
};
use futures::{future, TryFutureExt};
use std::sync::Arc;

pub struct Service {
    db_repo: Arc<dyn DBRepository>,
    bucket_repo: Arc<dyn BucketPort>,
}

impl Service {
    pub fn new(db_repo: Arc<dyn DBRepository>, bucket_repo: Arc<dyn BucketPort>) -> Self {
        Self {
            db_repo,
            bucket_repo,
        }
    }

    pub async fn create_complaint(
        &self,
        new_complaint: NewComplaint,
    ) -> Result<Complaint, ApiError> {
        // Check if driver exists or create a new one
        let driver = self.get_or_create_driver(&new_complaint).await?;

        // Add driver image if provided
        if let Some(driver_image_url) = new_complaint.driver_image {
            let driver_image = DriverImage {
                id: 0, // This will be set by the database
                driver_id: driver.id,
                image_url: driver_image_url,
            };
            self.db_repo.add_driver_image(&driver_image).await?;
        }

        // Create the complaint
        let complaint = Complaint {
            id: 0, // This will be set by the database
            driver_id: driver.id,
            location_id: new_complaint.location_id,
            taxi_application: new_complaint.taxi_application,
            description: new_complaint.description,
            created_at: chrono::Utc::now(),
        };
        let created_complaint = self.db_repo.create_complaint(&complaint).await?;

        // Add complaint images if provided
        if let Some(complaint_image_urls) = new_complaint.complaint_images {
            for image_url in complaint_image_urls {
                let complaint_image = ComplaintImage {
                    id: 0, // This will be set by the database
                    complaint_id: created_complaint.id,
                    image_url,
                };
                self.db_repo.add_complaint_image(&complaint_image).await?;
            }
        }

        Ok(created_complaint)
    }

    async fn get_or_create_driver(&self, new_complaint: &NewComplaint) -> Result<Driver, ApiError> {
        match self
            .db_repo
            .get_driver_by_name_and_license_plate(
                &new_complaint.taxi_driver_name,
                &new_complaint.taxi_license_plate,
            )
            .await
        {
            Ok(driver) => Ok(driver),
            Err(ApiError::NotFound(_)) => {
                // Driver not found, create a new one
                let new_driver = Driver {
                    id: 0, // This will be set by the database
                    name: new_complaint.taxi_driver_name.clone(),
                    license_plate: new_complaint.taxi_license_plate.clone(),
                };
                self.db_repo.create_driver(&new_driver).await
            }
            Err(e) => Err(e),
        }
    }

    pub async fn get_driver(&self, driver_id: i32) -> Result<Driver, ApiError> {
        self.db_repo.get_driver_by_id(driver_id).await
    }

    pub async fn get_driver_complaints(
        &self,
        driver_id: i32,
        pagination: &Pagination,
    ) -> Result<PaginatedRecord<Complaint>, ApiError> {
        // First, check if the driver exists
        self.db_repo.get_driver_by_id(driver_id).await?;

        // If the driver exists, get their complaints
        self.db_repo
            .get_complaints_for_driver(driver_id, pagination)
            .await
    }

    pub async fn get_driver_with_details(
        &self,
        driver_id: i32,
        pagination: &Pagination,
    ) -> Result<DriverWithDetails, ApiError> {
        let driver = self.db_repo.get_driver_by_id(driver_id).await?;
        let complaints = self
            .db_repo
            .get_complaints_for_driver(driver_id, pagination)
            .await?;
        let driver_images = self
            .db_repo
            .get_driver_images(
                driver_id,
                &Pagination {
                    page: 1,
                    per_page: 100,
                },
            )
            .await?;

        Ok(DriverWithDetails {
            driver,
            complaints,
            images: driver_images.items,
        })
    }

    pub async fn search_drivers(
        &self,
        query: &str,
        pagination: &Pagination,
    ) -> Result<PaginatedRecord<Driver>, ApiError> {
        self.db_repo.search_drivers(query, pagination).await
    }

    pub async fn search_drivers_with_images(
        &self,
        query: &str,
        pagination: &Pagination,
    ) -> Result<PaginatedRecord<DriverWithImages>, ApiError> {
        let drivers = self.db_repo.search_drivers(query, pagination).await?;

        let drivers_with_images: Vec<DriverWithImages> =
            future::try_join_all(drivers.items.into_iter().map(|driver| {
                self.db_repo
                    .get_driver_images(
                        driver.id,
                        &Pagination {
                            page: 1,
                            per_page: 5,
                        },
                    )
                    .map_ok(move |images| DriverWithImages {
                        driver,
                        images: images.items,
                    })
                    .map_err(ApiError::from)
            }))
            .await?;

        Ok(PaginatedRecord::new(
            drivers_with_images,
            drivers.total_items,
            drivers.page,
            drivers.per_page,
        ))
    }

    pub async fn generate_driver_image_upload_url(
        &self,
        driver_id: i32,
    ) -> Result<(String, String), ApiError> {
        // Check if the driver exists
        self.db_repo.get_driver_by_id(driver_id).await?;

        // Generate a unique file name
        let file_name = format!("driver_images/{}/{}", driver_id, uuid::Uuid::new_v4());

        // Generate the presigned URL
        self.bucket_repo.generate_upload_url(&file_name).await
    }

    pub async fn generate_image_upload_url(
        &self,
        prefix: &str,
    ) -> Result<(String, String), ApiError> {
        let file_name = format!("{}/{}", prefix, uuid::Uuid::new_v4());

        self.bucket_repo.generate_upload_url(&file_name).await
    }

    pub async fn get_complaint_with_images(
        &self,
        complaint_id: i32,
    ) -> Result<ComplaintWithImages, ApiError> {
        let complaint = self.db_repo.get_complaint_by_id(complaint_id).await?;

        let images = self
            .db_repo
            .get_complaint_images(
                complaint_id,
                &Pagination {
                    page: 1,
                    per_page: 100, // Adjust this value as needed
                },
            )
            .await?;

        Ok(ComplaintWithImages {
            complaint,
            images: images.items,
        })
    }
}
