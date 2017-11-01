use tomorrow_recuperator::Recuperator;
use recuperator_google_maps::*;
use recuperator_google_maps::models::Record;

use rocket_contrib::Json;

use ::api_response::ApiResponse;

#[get("/<address>")]
pub fn search(address: String) -> Json<ApiResponse<Record>> {
    let recuperator = GoogleMapsRecuperator::default();
    let request = GoogleMapsRequest::new(&address);
    let result = recuperator.compute(request);

    match result {
        Ok(response) => Json(ApiResponse::from(response.record)),
        Err(e) => Json(ApiResponse::from_err(e))
    }
}