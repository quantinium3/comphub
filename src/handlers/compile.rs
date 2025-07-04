use std::str::FromStr;

use crate::infra::{compile::compile_lang, error::InfraError};
use axum::Json;
use serde::{Deserialize, Serialize};

use super::error::ApiError;

#[derive(Serialize)]
pub struct CompilerResponse {
    result: String,
}

#[derive(Deserialize)]
pub struct CompilerRequest {
    lang: String,
    content: String,
    #[serde(default)]
    stdin: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Language {
    Python,
}

impl FromStr for Language {
    type Err = InfraError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "python" => Ok(Language::Python),
            _ => Err(InfraError::UnsupportedLanguage(
                format!("{} language is not supported", s).into(),
            )),
        }
    }
}

pub async fn compile(
    Json(payload): Json<CompilerRequest>,
) -> Result<Json<CompilerResponse>, ApiError> {
    payload.lang.parse::<Language>()?;
    let res = compile_lang(&payload.lang, &payload.content, &payload.stdin).await?;

    Ok(Json(CompilerResponse {
        result: res.to_string(),
    }))
}
