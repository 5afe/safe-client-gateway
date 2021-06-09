use chrono::Utc;
use rocket::fairing::{Fairing, Info, Kind};
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
        log::info!("MT::{}::{}::{}::{}", method, path_data, delta, status_code)
    }
}
