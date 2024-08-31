use std::str::FromStr;

use actix_web::ResponseError;
use serde::{Serialize, Deserialize};
use thiserror::Error;
use serde_json::json;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct Username(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Error)]
pub enum UsernameError {
    #[error("Username must be at least 3 characters long")]
    TooShort,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct Password(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Error)]
pub enum PasswordError {
    #[error("Password must be at least 8 characters long")]
    TooShort,
}

#[derive(Debug, Error)]
pub enum LoginRequestError {
    #[error("Username error: {0}")]
    UsernameError(#[from] UsernameError),
    #[error("Password error: {0}")]
    PasswordError(#[from] PasswordError),
}

impl ResponseError for LoginRequestError {
    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::BadRequest().json(json!({ "message": self.to_string() }))
    }
}

impl FromStr for Username {
    type Err = UsernameError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 3 {
            Err(UsernameError::TooShort)
        } else {
            Ok(Username(s.to_string()))
        }
    }
}

impl FromStr for Password {
    type Err = PasswordError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 8 {
            Err(PasswordError::TooShort)
        } else {
            Ok(Password(s.to_string()))
        }
    }
}

impl<'de> serde::Deserialize<'de> for Username {
    fn deserialize<D>(deserializer: D) -> Result<Username, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Username::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl<'de> serde::Deserialize<'de> for Password {
    fn deserialize<D>(deserializer: D) -> Result<Password, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Password::from_str(&s).map_err(serde::de::Error::custom)
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: Username,
    pub password: Password,
}

impl LoginRequest {
    pub fn from_str(username: &str, password: &str) -> Result<Self, LoginRequestError> {
        Ok(LoginRequest {
            username: Username::from_str(username)?,
            password: Password::from_str(password)?,
        })
    }
}
