use tomorrow_recuperator::Recuperator;
use recuperator_facebook::public::*;
use recuperator_facebook::public::models::People;

use rocket_contrib::Json;

use ::api_response::ApiResponse;

#[get("/public/<query>/<page>")]
pub fn public_search(query: String, page: u8) -> Json<ApiResponse<Vec<People>>> {
    let recuperator = FacebookPublicRecuperator::default();
    let request = FacebookPublicRequest::new(&query, page);
    let result = recuperator.compute(request);

    match result {
        Ok(response) => Json(ApiResponse::from(response.results)),
        Err(e) => Json(ApiResponse::from_err(e))
    }
}