use tomorrow_core::*;

use serde::ser::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct ApiResponse<T> {
    code: u8,
    error: Option<String>,
    result: Option<T>
}

impl <T> ApiResponse<T> where T: Serialize {

    pub fn from_err(error: Error) -> Self {
        ApiResponse {
            code: 1,
            error: Some(error.to_string()),
            result: None
        }
    }
}

impl <T> From<T> for ApiResponse<T> where T: Serialize {

    fn from(result: T) -> Self {
        ApiResponse {
            code: 0, 
            error: None,
            result: Some(result)
        }
    }
}

impl <T> From<Result<T>> for ApiResponse<T> where T: Serialize {

    fn from(result: Result<T>) -> Self {
        match result {
            Ok(result) => ApiResponse::from(result),
            Err(e) => ApiResponse::from_err(e)
        }
    }
}