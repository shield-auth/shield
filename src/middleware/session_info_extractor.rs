use axum::{
    body::Body,
    http::{HeaderMap, Request},
    middleware::Next,
    response::Response,
};
// use maxminddb::Reader;
use std::{net::IpAddr, sync::Arc};
use uaparser::{Parser, UserAgentParser};

// You might want to put this in a separate module
#[derive(Clone, Debug)]
pub struct SessionInfo {
    pub ip_address: IpAddr,
    pub user_agent: String,
    pub browser: String,
    pub browser_version: String,
    pub operating_system: String,
    pub device_type: String,
    pub country_code: String,
}

pub async fn session_info_middleware(mut req: Request<Body>, next: Next) -> Response {
    let headers = req.headers();
    let ip_address = extract_ip_from_headers(headers);
    let user_agent = req.headers().get("user-agent").and_then(|h| h.to_str().ok()).unwrap_or("").to_string();

    // Parse user agent
    let ua_parser = UserAgentParser::from_yaml("config/regexes.yml").expect("Failed to parse regexes.yml");
    let ua = ua_parser.parse(&user_agent);

    // Get country code using MaxMind GeoIP database
    // let reader = Reader::open("path/to/GeoLite2-Country.mmdb").unwrap();
    // let country_code = reader
    //     .lookup(addr.ip())
    //     .ok()
    //     .and_then(|r: maxminddb::geoip2::Country| r.country)
    //     .and_then(|c| c.iso_code)
    //     .unwrap_or("Unknown")
    //     .to_string();

    let session_info = SessionInfo {
        ip_address,
        browser: ua.device.family.to_string(),
        browser_version: format!("{:?}", ua.device.brand),
        operating_system: format!("{:?}", ua.os.family),
        device_type: format!("{:?}, {:?}, {:?}", ua.device.brand, ua.device.family, ua.device.model),
        country_code: "IN".to_string(),
        user_agent: user_agent.to_string(),
    };
    // println!("USER AGENT SESSION INFO: {:#?}, USER agent: {:#?}", session_info, ua);

    // Insert the updated AppState into the request extensions
    req.extensions_mut().insert(Arc::new(session_info));

    next.run(req).await
}

fn extract_ip_from_headers(headers: &HeaderMap) -> IpAddr {
    headers
        .get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.split(',').next())
        .and_then(|s| s.parse().ok())
        .or_else(|| headers.get("x-real-ip").and_then(|h| h.to_str().ok()).and_then(|s| s.parse().ok()))
        .unwrap_or(IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)))
}
