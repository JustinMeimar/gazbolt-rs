use url::Url;
use once_cell::sync::Lazy;

static CONFIG: Lazy<Config> = Lazy::new(|| Config::new());

pub struct Config {
    base_url: Url,
}

impl Config {
    fn new() -> Self {
        let base_url_str = option_env!("SERVER_URL")
            .unwrap_or("http://127.0.0.1:3000");

        let base_url = Url::parse(base_url_str)
            .unwrap_or_else(|_| {
                Url::parse("http://127.0.0.1:3000").expect("URL should be valid")
            });
        
        Self { base_url }
    }
}

pub fn create_url(path: &str) -> String {
    match CONFIG.base_url.join(path) {
        Ok(url) => url.to_string(),
        Err(_) => {
            #[cfg(debug_assertions)]
            web_sys::console::error_1(&format!("Failed to join URL path: {}", path).into());
            
            let cleaned_path = path.trim_start_matches('/');
            format!("{}/{}", CONFIG.base_url, cleaned_path)
        }
    }
}

