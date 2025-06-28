#[derive(Clone)]
pub struct Config {
    pub jwt_secret: String,
    pub jwt_expiry_hours: i64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            jwt_secret: "your-super-secret-jwt-key-change-in-production".to_string(),
            jwt_expiry_hours: 24,
        }
    }
}
