use std::str::FromStr;

use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Username(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Error)]
pub enum UsernameError {
    #[error("Username must be at least 3 characters long")]
    TooShort,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
