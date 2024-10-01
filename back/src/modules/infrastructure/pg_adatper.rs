use async_trait::async_trait;

use crate::{
    error::ApiError,
    modules::{port::DBRepository, Complaint, ComplaintImage, Driver, DriverImage, Location},
    utils::database::{PaginatedRecord, Pagination, PostgresRepository},
};

#[async_trait]
impl DBRepository for PostgresRepository {
    // Driver operations
    async fn create_driver(&self, driver: &Driver) -> Result<Driver, ApiError> {
        sqlx::query_as::<_, Driver>(
            "INSERT INTO drivers (name, license_plate) VALUES ($1, $2) RETURNING *",
        )
        .bind(&driver.name)
        .bind(&driver.license_plate)
        .fetch_one(&*self.pg_pool)
        .await
        .map_err(ApiError::DatabaseError)
    }

    async fn get_driver_by_id(&self, id: i32) -> Result<Driver, ApiError> {
        sqlx::query_as::<_, Driver>("SELECT * FROM drivers WHERE id = $1")
            .bind(id)
            .fetch_one(&*self.pg_pool)
            .await
            .map_err(|err| match err {
                sqlx::Error::RowNotFound => {
                    ApiError::NotFound(format!("Driver with id {} not found", id))
                }
                _ => ApiError::DatabaseError(err),
            })
    }

    async fn get_driver_by_license_plate(&self, license_plate: &str) -> Result<Driver, ApiError> {
        sqlx::query_as::<_, Driver>("SELECT * FROM drivers WHERE license_plate = $1")
            .bind(license_plate)
            .fetch_one(&*self.pg_pool)
            .await
            .map_err(|err| match err {
                sqlx::Error::RowNotFound => ApiError::NotFound(format!(
                    "Driver with license plate {} not found",
                    license_plate
                )),
                _ => ApiError::DatabaseError(err),
            })
    }

    async fn update_driver(&self, driver: &Driver) -> Result<Driver, ApiError> {
        sqlx::query_as::<_, Driver>(
            "UPDATE drivers SET name = $1, license_plate = $2 WHERE id = $3 RETURNING *",
        )
        .bind(&driver.name)
        .bind(&driver.license_plate)
        .bind(driver.id)
        .fetch_one(&*self.pg_pool)
        .await
        .map_err(ApiError::DatabaseError)
    }

    async fn delete_driver(&self, id: i32) -> Result<(), ApiError> {
        sqlx::query("DELETE FROM drivers WHERE id = $1")
            .bind(id)
            .execute(&*self.pg_pool)
            .await
            .map_err(ApiError::DatabaseError)?;
        Ok(())
    }

    // Driver Image operations
    async fn add_driver_image(&self, driver_image: &DriverImage) -> Result<DriverImage, ApiError> {
        sqlx::query_as::<_, DriverImage>(
            "INSERT INTO driver_images (driver_id, image_url) VALUES ($1, $2) RETURNING *",
        )
        .bind(driver_image.driver_id)
        .bind(&driver_image.image_url)
        .fetch_one(&*self.pg_pool)
        .await
        .map_err(ApiError::DatabaseError)
    }

    async fn get_driver_images(
        &self,
        driver_id: i32,
        pagination: &Pagination,
    ) -> Result<PaginatedRecord<DriverImage>, ApiError> {
        let offset = (pagination.page - 1) * pagination.per_page;

        let images = sqlx::query_as::<_, DriverImage>(
            "SELECT * FROM driver_images 
            WHERE driver_id = $1
            ORDER BY id 
            LIMIT $2 OFFSET $3",
        )
        .bind(driver_id)
        .bind(pagination.per_page as i64)
        .bind(offset as i64)
        .fetch_all(&*self.pg_pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        let total_items: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM driver_images WHERE driver_id = $1")
                .bind(driver_id)
                .fetch_one(&*self.pg_pool)
                .await
                .map_err(ApiError::DatabaseError)?;

        Ok(PaginatedRecord::new(
            images,
            total_items as u64,
            pagination.page,
            pagination.per_page,
        ))
    }

    async fn delete_driver_image(&self, id: i32) -> Result<(), ApiError> {
        sqlx::query("DELETE FROM driver_images WHERE id = $1")
            .bind(id)
            .execute(&*self.pg_pool)
            .await
            .map_err(ApiError::DatabaseError)?;
        Ok(())
    }

    // Complaint operations
    async fn create_complaint(&self, complaint: &Complaint) -> Result<Complaint, ApiError> {
        sqlx::query_as::<_, Complaint>(
            "INSERT INTO complaints (driver_id, location_id, taxi_application, description) 
            VALUES ($1, $2, $3, $4) RETURNING *",
        )
        .bind(complaint.driver_id)
        .bind(complaint.location_id)
        .bind(&complaint.taxi_application)
        .bind(&complaint.description)
        .fetch_one(&*self.pg_pool)
        .await
        .map_err(ApiError::DatabaseError)
    }

    async fn get_complaint_by_id(&self, id: i32) -> Result<Complaint, ApiError> {
        sqlx::query_as::<_, Complaint>("SELECT * FROM complaints WHERE id = $1")
            .bind(id)
            .fetch_one(&*self.pg_pool)
            .await
            .map_err(|err| match err {
                sqlx::Error::RowNotFound => {
                    ApiError::NotFound(format!("Complaint with id {} not found", id))
                }
                _ => ApiError::DatabaseError(err),
            })
    }

    async fn update_complaint(&self, complaint: &Complaint) -> Result<Complaint, ApiError> {
        sqlx::query_as::<_, Complaint>(
            "UPDATE complaints 
            SET driver_id = $1, location_id = $2, taxi_application = $3, description = $4 
            WHERE id = $5 RETURNING *",
        )
        .bind(complaint.driver_id)
        .bind(complaint.location_id)
        .bind(&complaint.taxi_application)
        .bind(&complaint.description)
        .bind(complaint.id)
        .fetch_one(&*self.pg_pool)
        .await
        .map_err(ApiError::DatabaseError)
    }

    async fn delete_complaint(&self, id: i32) -> Result<(), ApiError> {
        sqlx::query("DELETE FROM complaints WHERE id = $1")
            .bind(id)
            .execute(&*self.pg_pool)
            .await
            .map_err(ApiError::DatabaseError)?;
        Ok(())
    }

    async fn get_complaints_for_driver(
        &self,
        driver_id: i32,
        pagination: &Pagination,
    ) -> Result<PaginatedRecord<Complaint>, ApiError> {
        let offset = (pagination.page - 1) * pagination.per_page;

        let complaints = sqlx::query_as::<_, Complaint>(
            "SELECT * FROM complaints 
            WHERE driver_id = $1 AND published = true
            ORDER BY id 
            LIMIT $2 OFFSET $3",
        )
        .bind(driver_id)
        .bind(pagination.per_page as i64)
        .bind(offset as i64)
        .fetch_all(&*self.pg_pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        let total_items: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM complaints WHERE driver_id = $1 AND published = true",
        )
        .bind(driver_id)
        .fetch_one(&*self.pg_pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Ok(PaginatedRecord::new(
            complaints,
            total_items as u64,
            pagination.page,
            pagination.per_page,
        ))
    }

    // Complaint Image operations
    async fn add_complaint_image(
        &self,
        complaint_image: &ComplaintImage,
    ) -> Result<ComplaintImage, ApiError> {
        sqlx::query_as::<_, ComplaintImage>(
            "INSERT INTO complaint_images (complaint_id, image_url) VALUES ($1, $2) RETURNING *",
        )
        .bind(complaint_image.complaint_id)
        .bind(&complaint_image.image_url)
        .fetch_one(&*self.pg_pool)
        .await
        .map_err(ApiError::DatabaseError)
    }

    async fn get_complaint_images(
        &self,
        complaint_id: i32,
        pagination: &Pagination,
    ) -> Result<PaginatedRecord<ComplaintImage>, ApiError> {
        let offset = (pagination.page - 1) * pagination.per_page;

        let images = sqlx::query_as::<_, ComplaintImage>(
            "SELECT * FROM complaint_images 
            WHERE complaint_id = $1
            ORDER BY id 
            LIMIT $2 OFFSET $3",
        )
        .bind(complaint_id)
        .bind(pagination.per_page as i64)
        .bind(offset as i64)
        .fetch_all(&*self.pg_pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        let total_items: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM complaint_images WHERE complaint_id = $1")
                .bind(complaint_id)
                .fetch_one(&*self.pg_pool)
                .await
                .map_err(ApiError::DatabaseError)?;

        Ok(PaginatedRecord::new(
            images,
            total_items as u64,
            pagination.page,
            pagination.per_page,
        ))
    }

    async fn delete_complaint_image(&self, id: i32) -> Result<(), ApiError> {
        sqlx::query("DELETE FROM complaint_images WHERE id = $1")
            .bind(id)
            .execute(&*self.pg_pool)
            .await
            .map_err(ApiError::DatabaseError)?;
        Ok(())
    }

    // Location operations
    async fn get_location_by_id(&self, id: i32) -> Result<Location, ApiError> {
        sqlx::query_as::<_, Location>("SELECT * FROM locations WHERE id = $1")
            .bind(id)
            .fetch_one(&*self.pg_pool)
            .await
            .map_err(|err| match err {
                sqlx::Error::RowNotFound => {
                    ApiError::NotFound(format!("Location with id {} not found", id))
                }
                _ => ApiError::DatabaseError(err),
            })
    }

    async fn get_locations_by_country(
        &self,
        country: &str,
        pagination: &Pagination,
    ) -> Result<PaginatedRecord<Location>, ApiError> {
        let offset = (pagination.page - 1) * pagination.per_page;

        let locations = sqlx::query_as::<_, Location>(
            "SELECT * FROM locations 
            WHERE country = $1::country
            ORDER BY state 
            LIMIT $2 OFFSET $3",
        )
        .bind(country)
        .bind(pagination.per_page as i64)
        .bind(offset as i64)
        .fetch_all(&*self.pg_pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        let total_items: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM locations WHERE country = $1::country")
                .bind(country)
                .fetch_one(&*self.pg_pool)
                .await
                .map_err(ApiError::DatabaseError)?;

        Ok(PaginatedRecord::new(
            locations,
            total_items as u64,
            pagination.page,
            pagination.per_page,
        ))
    }

    async fn get_all_locations(
        &self,
        pagination: &Pagination,
    ) -> Result<PaginatedRecord<Location>, ApiError> {
        let offset = (pagination.page - 1) * pagination.per_page;

        let locations = sqlx::query_as::<_, Location>(
            "SELECT * FROM locations 
            ORDER BY country, state 
            LIMIT $1 OFFSET $2",
        )
        .bind(pagination.per_page as i64)
        .bind(offset as i64)
        .fetch_all(&*self.pg_pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        let total_items: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM locations")
            .fetch_one(&*self.pg_pool)
            .await
            .map_err(ApiError::DatabaseError)?;

        Ok(PaginatedRecord::new(
            locations,
            total_items as u64,
            pagination.page,
            pagination.per_page,
        ))
    }
    async fn search_drivers(
        &self,
        query: &str,
        pagination: &Pagination,
    ) -> Result<PaginatedRecord<Driver>, ApiError> {
        let offset = (pagination.page - 1) * pagination.per_page;

        let drivers = sqlx::query_as::<_, Driver>(
            "SELECT DISTINCT d.* FROM drivers d
            INNER JOIN complaints c ON d.id = c.driver_id
            WHERE (d.name ILIKE $1 OR d.license_plate ILIKE $1)
            AND c.published = true
            ORDER BY d.name 
            LIMIT $2 OFFSET $3",
        )
        .bind(format!("%{}%", query))
        .bind(pagination.per_page as i64)
        .bind(offset as i64)
        .fetch_all(&*self.pg_pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        let total_items: i64 = sqlx::query_scalar(
            "SELECT COUNT(DISTINCT d.id) FROM drivers d
            INNER JOIN complaints c ON d.id = c.driver_id
            WHERE (d.name ILIKE $1 OR d.license_plate ILIKE $1)
            AND c.published = true",
        )
        .bind(format!("%{}%", query))
        .fetch_one(&*self.pg_pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Ok(PaginatedRecord::new(
            drivers,
            total_items as u64,
            pagination.page,
            pagination.per_page,
        ))
    }
}
