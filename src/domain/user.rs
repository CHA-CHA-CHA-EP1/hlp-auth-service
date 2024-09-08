use serde::{Deserialize, Deserializer, Serialize};
use validator::{Validate, ValidationError};


#[derive(Debug, Serialize)]
pub enum Status {
    Active,
    Inactive,
    Err
}

#[derive(Debug, Serialize)]
pub enum Role {
    Admin,
    User,
    Err
}

impl<'de> Deserialize<'de> for Role {
    fn deserialize<D>(deserializer: D) -> Result<Role, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        match s.as_str() {
            "Admin" => Ok(Role::Admin),
            "User" => Ok(Role::User),
            _ => Ok(Role::Err),
        }
    }
}


impl<'de> Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> Result<Status, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        match s.as_str() {
            "Active" => Ok(Status::Active),
            "Inactive" => Ok(Status::Inactive),
            _ => Ok(Status::Err),
        }
    }
}

#[derive(Debug, Serialize, Validate, Deserialize)]
pub struct UserSignup {
    #[validate(email, custom(function = "validate_unique_email", code = "400", message = "Email already exists"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    pub first_name: String,
    pub last_name: String,

    #[validate(custom(function = "validate_user_role", message = "Invalid role"))]
    pub role: Role,

    #[validate(custom(function = "validate_user_status", code = "400", message = "Invalid status"))]
    pub status: Status,
}



fn validate_user_role(role: &Role) -> Result<(), ValidationError> {
    match role {
        Role::Admin | Role::User => Ok(()),
        _ => Err(ValidationError::new("Invalid role")),
    }
}

fn validate_unique_email(email: &str) -> Result<(), ValidationError> {
    // Check if email is unique in the database
    if email == "admin@gmail.com" {
        return Err(ValidationError::new("Email already exists"));
    }
    Ok(())
}

fn validate_user_status(status: &Status) -> Result<(), ValidationError> {
    // Check if status is valid
    match status {
        Status::Active | Status::Inactive => Ok(()),
        _ => Err(ValidationError::new("Invalid status")),
    }
}

