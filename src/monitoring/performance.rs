use crate::config;
use chrono::Utc;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::uri::Path;
use rocket::{Data, Request, Response};

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
        if rand::random::<f32>() <= config::log_threshold() {
            let request_path = request.uri().path();

            let chain_id = extract_chain_id(&request_path);

            let route = request
                .route()
                .map(|route| route.uri.to_string())
                .unwrap_or(request.uri().path().to_string());
            let cached = request
                .local_cache(|| Utc::now().timestamp_millis())
                .to_owned();
            let method = request.method().as_str();
            let status_code = response.status().code;
            let delta = Utc::now().timestamp_millis() - cached;
            log::info!(
                "MT::{}::{}::{}::{}::{}::{}",
                method,
                route,
                delta,
                status_code,
                request.uri().to_string(), // full path with query params
                chain_id
            );
        }
    }
}

pub(super) fn extract_chain_id(path: &Path) -> String {
    let chain_id = path.segments().get(2);
    let contains_chains = path.segments().get(1).map_or(false, |it| it == "chains");
    if contains_chains && chain_id.is_some() {
        chain_id.unwrap().to_string()
    } else {
        String::from("-1")
    }
}
