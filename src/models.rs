use serde::{Deserialize, Serialize};

/// Represents a user object.
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}

/// Represents a generic API response with data.
#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T> {
    pub data: T,
}