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

impl TryFrom<String> for Username {
    type Error = UsernameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() < 3 {
            Err(UsernameError::TooShort)
        } else {
            Ok(Username(value))
        }
    }
}

impl TryFrom<String> for Password {
    type Error = PasswordError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() < 8 {
            Err(PasswordError::TooShort)
        } else {
            Ok(Password(value))
        }
    }
}

// API: /auth/login
// Method: POST
// Body: { "username": "john_doe", "password": "password123" }

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
