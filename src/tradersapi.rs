pub mod api {
    pub struct TradersApi {
        api_url_root: String,
        api_suburl_register: String,
        api_suburl_status: String,
    }

    impl TradersApi {
        // Immutable access to api_url_root via getter
        pub fn api_url_root(&self) -> &str {
            &self.api_url_root
        }

        // Immutable access to api_suburl_register via getter
        pub fn api_suburl_register(&self) -> &str {
            &self.api_suburl_register
        }

        // Immutable access to api_suburl_status via getter
        pub fn api_suburl_status(&self) -> &str {
            &self.api_suburl_status
        }
    }

    pub fn init_traders_api() -> TradersApi {
        // Initialize TradersApi struct with default values
        TradersApi {
            api_url_root: "https://api.spacetraders.io/v2/".to_string(),
            api_suburl_register: "register".to_string(),
            api_suburl_status: "my/agent".to_string(),
        }
    }
}
