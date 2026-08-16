#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
    pub admin_email: Option<String>,
    pub admin_password: Option<String>,
    pub admin_nickname: Option<String>,
    pub allow_registration: bool,
}

impl Config {
    pub fn from_env() -> Self {
        let admin_nickname = std::env::var("ADMIN_NICKNAME")
            .unwrap_or_else(|_| "admin".to_string());
        let admin_nickname = if admin_nickname.is_empty() {
            None
        } else {
            Some(admin_nickname)
        };

        Self {
            host: std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "5231".to_string())
                .parse()
                .expect("PORT must be a valid u16"),
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "shcut-default-secret-change-me".to_string()),
            admin_email: std::env::var("ADMIN_EMAIL").ok().filter(|s| !s.is_empty()),
            admin_password: std::env::var("ADMIN_PASSWORD").ok().filter(|s| !s.is_empty()),
            admin_nickname,
            allow_registration: std::env::var("ALLOW_REGISTRATION")
                .unwrap_or_else(|_| "true".to_string())
                .to_lowercase()
                != "false",
        }
    }
}
