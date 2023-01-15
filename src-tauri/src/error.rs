use anyhow::Error;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInfo {
    error_chain: Vec<String>,
    backtrace: String,
}

pub fn to_err(error: Error) -> ErrorInfo {
    return ErrorInfo {
        error_chain: error.chain().map(ToString::to_string).collect(),
        backtrace: error.backtrace().to_string(),
    };
}
