use std::{fmt::Display, fmt::Formatter, error::Error, any::Any};

// JsbError struct - Error struct
#[derive(Debug)]
pub struct JsbError {
    pub code: String,
    pub message: String,
}

impl Display for JsbError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for JsbError {}

impl JsbError {
    // convert any error to jsb error
    pub fn from_any(err: &dyn Any, _code: Option<&str>) -> JsbError {
        let mut code = _code.unwrap_or("500");

        if let Some(err) = err.downcast_ref::<JsbError>() {
            // if _code is not provided, use the code from the error
            if _code.is_none() {
                code = &err.code;
            }

            JsbError {
                code: code.to_string(),
                message: err.message.to_string(),
            }
        } else if let Some(err) = err.downcast_ref::<reqwest::Error>() {
            JsbError {
                code: code.to_string(),
                message: err.to_string(),
            }
        } else if let Some(err) = err.downcast_ref::<serde_json::Error>() {
            JsbError {
                code: code.to_string(),
                message: err.to_string(),
            }
        } else {
            JsbError {
                code: code.to_string(),
                message: "Unknown error".to_string(),
            }
        }
    }
}

// API Error struct
// #[derive(Debug)]
// pub struct ApiError {
//     pub code: String,
//     pub message: String,
// }

// Api Error Response struct
#[derive(Debug)]
pub struct ApiErrorResponse {
    pub error: JsbError,
}


// ====== Shared Errors ======
pub fn err_invalid_json() -> JsbError {
    JsbError {
        code: String::from("invalid_json_content"),
        message: "Content is not a valid JSON string".to_string(),
    }
}