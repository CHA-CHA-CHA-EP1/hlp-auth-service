use std::str::FromStr;

use serde::{Serialize, Deserialize};
use thiserror::Error;

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
