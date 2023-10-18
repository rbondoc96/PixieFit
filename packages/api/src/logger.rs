use std::time::SystemTime;

use axum::http::{Method, Uri};
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;

use crate::root::errors::ClientError;
use crate::{Context, Error, Result};

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLog {
    // possibly include request UUID
    timestamp: String, // should be ISO8601

    path: String,
    method: String,

    error: Option<String>,
    client_error: Option<String>,
    error_data: Option<Value>,
}

pub struct Logger;

impl Logger {
    pub async fn log_request(
        context: Option<Context>,
        method: Method,
        uri: Uri,
        error: Option<&Error>,
        client_error: Option<ClientError>,
    ) -> Result<()> {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let error_type = error.map(|err| err.as_ref().to_string());
        let error_data = serde_json::to_value(error)
            .ok()
            .and_then(|mut value| value.get_mut("data").map(|value| value.take()));

        let log_line = RequestLog {
            timestamp: timestamp.to_string(),
            path: uri.to_string(),
            method: method.to_string(),
            error: error_type,
            client_error: client_error.map(|err| err.as_ref().to_string()),
            error_data,
        };

        log::info!("->> [REQUEST_LOG]\n{}\n", json!(log_line));

        Ok(())
    }
}
