use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::utils::database::PaginatedRecord;

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "country", rename_all = "snake_case")]
pub enum Country {
    Peru,
    Mexico,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Location {
    pub id: i32,
    pub country: Country,
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Driver {
    pub id: i32,
    pub name: String,
    pub license_plate: String,
}

impl Driver {
    pub fn new(name: &str, license_plate: &str) -> Self {
        Self {
            id: 0,
            name: name.to_string(),
            license_plate: license_plate.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DriverImage {
    pub id: i32,
    pub driver_id: i32,
    pub image_url: String,
}

impl DriverImage {
    pub fn new(driver_id: i32, image_url: &str) -> Self {
        Self {
            id: 0,
            driver_id,
            image_url: image_url.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Complaint {
    pub id: i32,
    pub driver_id: i32,
    pub location_id: i32,
    pub taxi_application: String,
    pub description: String,
}

impl Complaint {
    pub fn new(
        driver_id: i32,
        location_id: i32,
        taxi_application: &str,
        description: &str,
    ) -> Self {
        Self {
            id: 0,
            driver_id,
            location_id,
            taxi_application: taxi_application.to_string(),
            description: description.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ComplaintImage {
    pub id: i32,
    pub complaint_id: i32,
    pub image_url: String,
}

impl ComplaintImage {
    pub fn new(complaint_id: i32, image_url: &str) -> Self {
        Self {
            id: 0,
            complaint_id,
            image_url: image_url.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewComplaint {
    pub description: String,
    pub taxi_driver_name: String,
    pub taxi_license_plate: String,
    pub location_id: i32,
    pub taxi_application: String,
    pub driver_image: Option<String>,
    pub complaint_images: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DriverWithDetails {
    pub driver: Driver,
    pub complaints: PaginatedRecord<Complaint>,
    pub images: Vec<DriverImage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DriverWithImages {
    pub driver: Driver,
    pub images: Vec<DriverImage>,
}
