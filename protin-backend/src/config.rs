use std::env;

use anyhow::Context;

#[derive(Clone, Debug)]
pub struct Config {
    database_url: String,
    s3_region: String,
    s3_endpoint: String,
    s3_bucket_name: String,
    s3_bucket_lifcycle_id: String,
    s3_bucket_expiration: i32,
    web_port: u16,
}

const DEFAULT_S3_BUCKET_NAME: &str = "protin-files";
const DEFAULT_S3_BUCKET_LIFECYCLE_ID: &str = "protin-files-expiration-lifecycle";
const DEFAULT_S3_BUCKET_EXPIRATION: u32 = 1;
const DEFAULT_WEB_PORT: u32 = 8080;

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let database_url = if let Ok(url) = env::var("DATABASE_URL") {
            url
        } else {
            let user = env::var("POSTGRES_USER").context(
                "POSTGRES_USER environment variable must be set if DATABASE_URL is not set.",
            )?;
            let password = env::var("POSTGRES_PASSWORD").context(
                "POSTGRES_PASSWORD environment variable must be set if DATABASE_URL is not set.",
            )?;
            let db = env::var("POSTGRES_DB").context(
                "POSTGRES_DB environment variable must be set if DATABASE_URL is not set.",
            )?;
            let host = env::var("POSTGRES_HOST").unwrap_or("localhost".to_string());

            format!("postgres://{user}:{password}:{host}/{db}")
        };

        let s3_region =
            env::var("S3_REGION").context("S3_REGION environment variable must be set.")?;
        let s3_endpoint =
            env::var("S3_ENDPOINT").context("S3_ENDPOINT environment variable must be set.")?;

        let s3_bucket_name =
            env::var("S3_BUCKET_NAME").unwrap_or(DEFAULT_S3_BUCKET_NAME.to_string());
        let s3_bucket_lifcycle_id = env::var("S3_BUCKET_LIFECYCLE_ID")
            .unwrap_or(DEFAULT_S3_BUCKET_LIFECYCLE_ID.to_string());

        let s3_bucket_expiration_string =
            env::var("S3_BUCKET_EXPIRATION").unwrap_or(DEFAULT_S3_BUCKET_EXPIRATION.to_string());
        let s3_bucket_expiration = s3_bucket_expiration_string
            .parse()
            .context("S3_BUCKET_EXPIRATION needs to be signed integer value")?;

        let web_port_string = env::var("WEB_PORT").unwrap_or(DEFAULT_WEB_PORT.to_string());
        let web_port = web_port_string
            .parse()
            .context("WEB_PORT needs to be unsigned integer value")?;

        Ok(Self {
            database_url,
            s3_region,
            s3_endpoint,
            s3_bucket_name,
            s3_bucket_lifcycle_id,
            s3_bucket_expiration,
            web_port,
        })
    }

    pub fn database_url(&self) -> String {
        self.database_url.to_string()
    }

    pub fn s3_region(&self) -> String {
        self.s3_region.to_string()
    }

    pub fn s3_endpoint(&self) -> String {
        self.s3_endpoint.to_string()
    }

    pub fn s3_bucket_name(&self) -> String {
        self.s3_bucket_name.to_string()
    }

    pub fn s3_bucket_lifcycle_id(&self) -> String {
        self.s3_bucket_lifcycle_id.to_string()
    }

    pub fn s3_bucket_expiration(&self) -> i32 {
        self.s3_bucket_expiration
    }

    pub fn web_port(&self) -> u16 {
        self.web_port
    }
}
