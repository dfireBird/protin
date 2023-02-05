use std::io::Read;

use actix_easy_multipart::{tempfile::Tempfile, MultipartForm};
use actix_web::{
    get, post,
    web::{self, ServiceConfig},
    Error, HttpResponse,
};
use log::error;

use crate::{paste, AppState};

pub fn pastes_config(cfg: &mut ServiceConfig) {
    cfg.service(get_paste_route);
    cfg.service(create_paste_route);
}

#[derive(Debug, MultipartForm)]
struct FileUpload {
    file: Tempfile,
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
    file_upload: MultipartForm<FileUpload>,
) -> Result<HttpResponse, Error> {
    let mut file_data = Vec::new();
    let mut file = file_upload.file.file.as_file();
    if let Err(err) = file.read_to_end(&mut file_data) {
        error!("Error: {:#}", err);
        return Ok(HttpResponse::InternalServerError().body(format!("{}", err)));
    }

    match paste::create_paste(data.clone(), &file_data).await {
        Ok(paste) => return Ok(HttpResponse::Ok().json(paste)),
        Err(err) => {
            error!("Error: {:#}", err);
            return Ok(HttpResponse::InternalServerError().body(format!("{}", err)));
        }
    }
}
