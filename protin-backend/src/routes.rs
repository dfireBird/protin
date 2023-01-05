use actix_multipart::Multipart;
use actix_web::{
    post,
    web::{self, ServiceConfig},
    Error, HttpResponse,
};
use futures_util::TryStreamExt;
use log::error;

use crate::{paste, AppState};

pub fn pastes_config(cfg: &mut ServiceConfig) {
    cfg.service(create_paste_route);
}

#[post("/paste")]
async fn create_paste_route(
    data: web::Data<AppState>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    while let Some(mut field) = payload.try_next().await? {
        let mut file_data = Vec::new();
        while let Some(chunk) = field.try_next().await? {
            file_data.append(&mut chunk.to_vec())
        }
        match paste::create_paste(data.clone(), &file_data).await {
            Ok(paste) => return Ok(HttpResponse::Ok().json(paste)),
            Err(err) => {
                error!("Error: {:#}", err);
                return Ok(HttpResponse::InternalServerError().body(format!("{}", err)));
            }
        }
    }
    Ok(HttpResponse::BadRequest().body("No fields available in the request"))
}
