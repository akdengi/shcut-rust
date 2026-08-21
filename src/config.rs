#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
    pub admin_email: Option<String>,
    pub admin_password: Option<String>,
    pub admin_nickname: Option<String>,
    pub allow_registration: bool,
    pub smtp_host: Option<String>,
    pub smtp_port: u16,
    pub smtp_user: Option<String>,
    pub smtp_password: Option<String>,
    pub smtp_from: Option<String>,
    pub app_url: Option<String>,
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
            smtp_host: std::env::var("SMTP_HOST").ok().filter(|s| !s.is_empty()),
            smtp_port: std::env::var("SMTP_PORT")
                .unwrap_or_else(|_| "587".to_string())
                .parse()
                .unwrap_or(587),
            smtp_user: std::env::var("SMTP_USER").ok().filter(|s| !s.is_empty()),
            smtp_password: std::env::var("SMTP_PASSWORD").ok().filter(|s| !s.is_empty()),
            smtp_from: std::env::var("SMTP_FROM").ok().filter(|s| !s.is_empty()),
            app_url: std::env::var("APP_URL").ok().filter(|s| !s.is_empty()),
        }
    }

    pub fn smtp_configured(&self) -> bool {
        self.smtp_host.is_some() && self.smtp_user.is_some() && self.smtp_password.is_some()
    }
}
