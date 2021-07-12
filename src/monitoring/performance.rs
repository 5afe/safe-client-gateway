use chrono::Utc;
use lazy_static::lazy_static;
use regex::Regex;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Request, Response};

lazy_static! {
    static ref CHAIN_ID_PATH_PATTERN: Regex = Regex::new(r"/v1/chains/d{+}/+").unwrap();
}

pub struct PerformanceMonitor();

#[rocket::async_trait]
impl Fairing for PerformanceMonitor {
    fn info(&self) -> Info {
        Info {
            name: "PerformanceMonitor",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _data: &mut Data<'_>) {
        request.local_cache(|| Utc::now().timestamp_millis());
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let raw_path_string = request.uri().path().to_string();
        let raw_path = request.uri().path();

        log::error!(
            "raw path: {:#?}",
            &raw_path.segments().find(|segment| segment == &"chains")
        );
        let chain_id = if raw_path_string.contains("/v1/chains/*/*") {
            raw_path_string.find("/v1/chains//")
        } else {
            None
        };

        let path_data = request
            .route()
            .map(|route| route.uri.to_string())
            .unwrap_or(request.uri().path().to_string());
        let cached = request
            .local_cache(|| Utc::now().timestamp_millis())
            .to_owned();
        let method = request.method().as_str();
        let status_code = response.status().code;
        let delta = Utc::now().timestamp_millis() - cached;
        if chain_id.is_some() {
            log::info!(
                "MT::{}::{}::{}::{}::{}::{}",
                method,
                raw_path_string,
                delta,
                status_code,
                path_data,
                chain_id.unwrap()
            )
        } else {
            log::info!("MT::{}::{}::{}::{}", method, path_data, delta, status_code)
        }
    }
}
