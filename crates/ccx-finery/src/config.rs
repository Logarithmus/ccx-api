use url_macro::url;

use url::Url;

pub struct ConnectionConfig {
    pub(crate) api_base: Url,
}

impl ConnectionConfig {
    pub fn new(api_base: Url) -> Self {
        Self { api_base }
    }

    pub fn prod() -> Self {
        Self::new(url!("https://trade.finerymarkets.com/api"))
    }

    pub fn dev() -> Self {
        Self::new(url!("https://test.finerymarkets.com/api"))
    }
}
