use actix_multipart::Multipart;
use actix_web::{
    get, post,
    web::{self, ServiceConfig},
    Error, HttpResponse,
};
use log::error;
use tokio_stream::StreamExt;

use crate::{paste, AppState};

pub fn pastes_config(cfg: &mut ServiceConfig) {
    cfg.service(get_paste_route);
    cfg.service(create_paste_route);
}

#[get("/paste/{paste_id}")]
async fn get_paste_route(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let paste_id = path.into_inner();
    match paste::get_paste(data, paste_id).await {
        Ok(Some(data)) => Ok(HttpResponse::Ok().body(data)),
        Ok(None) => Ok(HttpResponse::NotFound().body("Paste not found.")),
        Err(err) => {
            error!("Error: {:#}", err);
            Ok(HttpResponse::InternalServerError().body(format!("{err}")))
        }
    }
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
