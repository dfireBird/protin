use std::io::Read;

use actix_multipart::form::{MultipartForm, tempfile::TempFile};
use actix_web::{Error, HttpResponse, get, post, web};
use base64::{
    Engine as _,
    engine::general_purpose::{STANDARD as BASE64_STANDARD, URL_SAFE as BASE64_URL_SAFE},
};
use log::error;

use crate::{AppState, paste};

pub fn pastes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_paste_route);
    cfg.service(create_paste_route);
}

#[derive(Debug, MultipartForm)]
struct FileUpload {
    file: TempFile,
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

    let Ok(file_data) = String::from_utf8(file_data) else {
        return Ok(
            HttpResponse::BadRequest().body("File content must be UTF-8 encoded. The uploaded file contains invalid or unsupported encoding.")
        );
    };

    let is_valid_base64 =
        BASE64_STANDARD.decode(&file_data).is_ok() || BASE64_URL_SAFE.decode(&file_data).is_ok();
    if is_valid_base64 {
        return Ok(HttpResponse::BadRequest().body("File content appears to be Base64 encoded. Upload of Base64 encoded files are not allowed"));
    }

    match paste::create_paste(data.clone(), file_data.as_bytes()).await {
        Ok(paste) => Ok(HttpResponse::Ok().json(paste)),
        Err(err) => {
            error!("Error: {:#}", err);
            Ok(HttpResponse::InternalServerError().body(format!("{}", err)))
        }
    }
}
