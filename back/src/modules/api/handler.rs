use actix_web::{web, HttpResponse};
use serde::Deserialize;
use std::sync::Arc;

use crate::error::ApiError;
use crate::modules::{NewComplaint, Service};
use crate::utils::database::Pagination;

#[derive(Deserialize)]
pub struct CreateComplaintRequest {
    pub description: String,
    pub taxi_driver_name: String,
    pub taxi_license_plate: String,
    pub location_id: i32,
    pub taxi_application: String,
    pub driver_image: Option<String>,
    pub complaint_images: Option<Vec<String>>,
}

pub async fn create_complaint(
    service: web::Data<Arc<Service>>,
    req: web::Json<CreateComplaintRequest>,
) -> Result<HttpResponse, ApiError> {
    let new_complaint = NewComplaint {
        description: req.description.clone(),
        taxi_driver_name: req.taxi_driver_name.clone(),
        taxi_license_plate: req.taxi_license_plate.clone(),
        location_id: req.location_id,
        taxi_application: req.taxi_application.clone(),
        driver_image: req.driver_image.clone(),
        complaint_images: req.complaint_images.clone(),
    };

    let created_complaint = service.create_complaint(new_complaint).await?;
    Ok(HttpResponse::Ok().json(created_complaint))
}

pub async fn get_driver(
    service: web::Data<Arc<Service>>,
    driver_id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let driver = service.get_driver(*driver_id).await?;
    Ok(HttpResponse::Ok().json(driver))
}

pub async fn get_driver_complaints(
    service: web::Data<Arc<Service>>,
    driver_id: web::Path<i32>,
    web::Query(pagination): web::Query<Pagination>,
) -> Result<HttpResponse, ApiError> {
    let complaints = service
        .get_driver_complaints(*driver_id, &pagination)
        .await?;
    Ok(HttpResponse::Ok().json(complaints))
}

pub async fn get_driver_with_details(
    service: web::Data<Arc<Service>>,
    driver_id: web::Path<i32>,
    web::Query(pagination): web::Query<Pagination>,
) -> Result<HttpResponse, ApiError> {
    let driver_details = service
        .get_driver_with_details(*driver_id, &pagination)
        .await?;
    Ok(HttpResponse::Ok().json(driver_details))
}

#[derive(Deserialize)]
pub struct SearchDriversQuery {
    pub query: String,
    pub page: u32,
    pub per_page: u32,
}

pub async fn search_drivers(
    service: web::Data<Arc<Service>>,
    web::Query(query): web::Query<SearchDriversQuery>,
) -> Result<HttpResponse, ApiError> {
    let pagination = Pagination {
        page: query.page,
        per_page: query.per_page,
    };
    let drivers = service.search_drivers(&query.query, &pagination).await?;
    Ok(HttpResponse::Ok().json(drivers))
}

pub async fn search_drivers_with_images(
    service: web::Data<Arc<Service>>,
    web::Query(query): web::Query<SearchDriversQuery>,
) -> Result<HttpResponse, ApiError> {
    let pagination = Pagination {
        page: query.page,
        per_page: query.per_page,
    };
    let drivers = service
        .search_drivers_with_images(&query.query, &pagination)
        .await?;
    Ok(HttpResponse::Ok().json(drivers))
}

#[derive(Deserialize)]
pub struct GenerateImageUploadUrlRequest {
    pub prefix: String,
}

pub async fn generate_image_upload_url(
    service: web::Data<Arc<Service>>,
    req: web::Json<GenerateImageUploadUrlRequest>,
) -> Result<HttpResponse, ApiError> {
    let (upload_url, public_url) = service.generate_image_upload_url(&req.prefix).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "upload_url": upload_url,
        "public_url": public_url
    })))
}

pub async fn get_complaint_with_images(
    service: web::Data<Arc<Service>>,
    complaint_id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let complaint_with_images = service.get_complaint_with_images(*complaint_id).await?;
    Ok(HttpResponse::Ok().json(complaint_with_images))
}

#[derive(Deserialize)]
pub struct SearchDriversWithDetailsQuery {
    pub query: String,
    pub page: u32,
    pub per_page: u32,
    pub complaints_page: u32,
    pub complaints_per_page: u32,
}

pub async fn search_drivers_with_details(
    service: web::Data<Arc<Service>>,
    web::Query(query): web::Query<SearchDriversWithDetailsQuery>,
) -> Result<HttpResponse, ApiError> {
    let pagination = Pagination {
        page: query.page,
        per_page: query.per_page,
    };
    let complaints_pagination = Pagination {
        page: query.complaints_page,
        per_page: query.complaints_per_page,
    };
    let drivers = service
        .search_drivers_with_details(&query.query, &pagination, &complaints_pagination)
        .await?;
    Ok(HttpResponse::Ok().json(drivers))
}
