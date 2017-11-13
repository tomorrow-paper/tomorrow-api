use rocket::Request;
use rocket_contrib::Json;

use ::api_response::ApiResponse;

#[derive(Serialize, Clone, Debug)]
struct HttpError {
    method: String,
    path: String,
    timestamp: i64
}

impl HttpError {

    pub fn new(request: &Request) -> Self {
        let current_time = ::time::get_time();
        let timestamp = (current_time.sec * 1000) + (current_time.nsec as i64 / 1000 / 1000);

        HttpError {
            method: request.method().to_string(),
            path: String::from(request.uri().path()),
            timestamp: timestamp
        }
    }
}

#[error(404)]
fn not_found(req: &Request) -> Json<ApiResponse<HttpError>> {
    let error = HttpError::new(req);
    Json(ApiResponse::as_err("Not Found", error))
}

#[error(500)]
fn internal_server_error(req: &Request) -> Json<ApiResponse<HttpError>> {
    let error = HttpError::new(req);
    Json(ApiResponse::as_err("Internal Server Error", error))
}