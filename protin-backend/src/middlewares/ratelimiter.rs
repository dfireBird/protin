use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
    time::Duration,
};

use actix_governor::{KeyExtractor, SimpleKeyExtractionError};

const IP_KEY_EXTRACT_ERROR_MESSAGE: &'static str = "Could not extract real IP address from request";
const IP_KEY_PARSE_ERROR_MESSAGE: &'static str = "Could not parse real IP address from request";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RealIpKeyExtractor {
    pub reverse_proxy_ip: IpAddr,
}

impl KeyExtractor for RealIpKeyExtractor {
    type Key = IpAddr;

    type KeyExtractionError = SimpleKeyExtractionError<&'static str>;

    fn extract(
        &self,
        req: &actix_web::dev::ServiceRequest,
    ) -> Result<Self::Key, Self::KeyExtractionError> {
        let peer_ip = req.peer_addr().map(|socket| socket.ip());
        let connection_info = req.connection_info();

        let ip_addr_mapper = |ip_str| {
            SocketAddr::from_str(ip_str)
                .map(|socket| socket.ip())
                .or_else(|_| IpAddr::from_str(ip_str))
                .map_err(|_| SimpleKeyExtractionError::new(IP_KEY_PARSE_ERROR_MESSAGE))
        };

        match peer_ip {
            Some(peer_ip) if peer_ip == self.reverse_proxy_ip => connection_info
                .realip_remote_addr()
                .ok_or_else(|| SimpleKeyExtractionError::new(IP_KEY_EXTRACT_ERROR_MESSAGE))
                .and_then(ip_addr_mapper),
            _ => connection_info
                .peer_addr()
                .ok_or_else(|| SimpleKeyExtractionError::new(IP_KEY_EXTRACT_ERROR_MESSAGE))
                .and_then(ip_addr_mapper),
        }
    }
}

const ONE_SEC_IN_NS: u128 = Duration::from_secs(1).as_nanos();

pub fn get_ns_per_request(request_per_second: u32) -> u64 {
    (ONE_SEC_IN_NS / (request_per_second as u128)) as u64
}
