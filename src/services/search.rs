use tomorrow_recuperator::Recuperator;
use recuperator_google::*;
use recuperator_google::models::SearchResult;

use rocket_contrib::Json;

use ::api_response::ApiResponse;

#[get("/<query>")]
pub fn search(query: String) -> Json<ApiResponse<Vec<SearchResult>>> {
    let recuperator = GoogleRecuperator::default();
    let request = GoogleRequest::new(&query);
    let result = recuperator.compute(request);

    match result {
        Ok(response) => Json(ApiResponse::from(response.results)),
        Err(e) => Json(ApiResponse::from_err(e))
    }
}