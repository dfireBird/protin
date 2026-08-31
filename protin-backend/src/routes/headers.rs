use actix_web::{
    HttpMessage,
    error::ParseError,
    http::header::{Header, HeaderName, HeaderValue, InvalidHeaderValue, TryIntoHeaderValue},
};

pub struct XApiKey(pub String);

impl TryIntoHeaderValue for XApiKey {
    type Error = InvalidHeaderValue;

    fn try_into_value(self) -> Result<HeaderValue, Self::Error> {
        HeaderValue::from_str(&self.0)
    }
}

impl Header for XApiKey {
    fn name() -> HeaderName {
        HeaderName::from_static("x-api-key")
    }

    fn parse<M: HttpMessage>(msg: &M) -> Result<Self, ParseError> {
        let header = msg
            .headers()
            .get(Self::name())
            .ok_or_else(|| ParseError::Header)?;

        let value = header.to_str().map_err(|_| ParseError::Header)?.to_string();

        Ok(XApiKey(value))
    }
}
