use async_trait::async_trait;

use crate::domain::auth::{Password, Username};

#[async_trait]
pub trait AuthService {
    async fn login(&self, username: Username, password: Password) -> Result<String, String>;
    async fn logout(&self) -> Result<(), String>;
}

#[derive(Clone)]
pub struct AuthServiceImpl {}

impl AuthServiceImpl {
    pub fn new() -> Self {
        AuthServiceImpl {}
    }
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn login(&self, username: Username, password: Password) -> Result<String, String> {
        if username.as_str() != "admin" {
            println!("Invalid username: {:?}", username);
            return Err("Invalid username".to_string());
        }

        // Implement login logic here
        Ok("token".to_string())
    }

    async fn logout(&self) -> Result<(), String> {
        // Implement logout logic here
        Ok(())
    }
}
