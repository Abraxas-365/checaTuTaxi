use actix_web::web;
use handler::{
    create_complaint, generate_image_upload_url, get_complaint_with_images, get_driver,
    get_driver_complaints, get_driver_with_details, search_drivers, search_drivers_with_images,
};

mod handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/taxi")
            .route(
                "/generate-image-upload-url",
                web::post().to(generate_image_upload_url),
            )
            .route("/complaint", web::post().to(create_complaint))
            .route(
                "/complaint/{complaint_id}",
                web::get().to(get_complaint_with_images),
            )
            .route("/driver/{driver_id}", web::get().to(get_driver))
            .route(
                "/driver/{driver_id}/complaints",
                web::get().to(get_driver_complaints),
            )
            .route(
                "/driver/{driver_id}/details",
                web::get().to(get_driver_with_details),
            )
            .route("/drivers/search", web::get().to(search_drivers))
            .route(
                "/drivers/search/with-images",
                web::get().to(search_drivers_with_images),
            ),
    );
}
