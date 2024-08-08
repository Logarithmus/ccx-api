mod error;
mod method;
mod request;
pub mod spot;
mod version;
pub mod wallet;
pub mod withdrawal;

pub use error::*;
pub use method::*;
pub use request::*;
pub use version::*;

pub const API_BASE: &str = "https://api.gateio.ws/api/";

#[cfg(feature = "with_network")]
pub use with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    use ccx_api_lib::GateApiCred;
    use ccx_api_lib::Proxy;
    use spot::SpotApi;

    pub use super::*;
    use crate::client::config::GateApiConfig;
    use crate::client::config::CCX_GATE_API_PREFIX;
    use crate::client::rest::GateRestClient;
    use crate::client::rest::RequestError;
    use crate::client::signer::GateSigner;

    #[derive(Clone)]
    pub struct GateApi<S>
    where
        S: GateSigner,
    {
        pub client: GateRestClient<S>,
    }

    impl<S> GateApi<S>
    where
        S: GateSigner,
    {
        pub fn new(signer: S, proxy: Option<Proxy>) -> GateApi<S> {
            let api_base = API_BASE.parse().unwrap();
            GateApi::with_config(GateApiConfig::new(signer, api_base, proxy))
        }

        pub fn from_env() -> GateApi<GateApiCred> {
            Self::from_env_with_prefix(CCX_GATE_API_PREFIX)
        }

        pub fn from_env_with_prefix(prefix: &str) -> GateApi<GateApiCred> {
            // FIXME prefix
            let proxy = Proxy::from_env_with_prefix(prefix);
            log::debug!(
                "from_env_with_prefix proxy :: {:?}",
                proxy.as_ref().map(|p| (&p.host, p.port))
            );
            GateApi::new(GateApiCred::from_env_with_prefix(prefix), proxy)
        }

        pub fn with_config(config: GateApiConfig<S>) -> GateApi<S> {
            let client = GateRestClient::new(config);
            GateApi { client }
        }

        pub async fn request<R: Request>(
            &self,
            path: &str,
            request: &R,
        ) -> Result<R::Response, RequestError> {
            let resp = if R::IS_PUBLIC {
                self.client.prepare_rest(path, request).call_unsigned().await?
            } else {
                let signed = self.client.prepare_rest(path, request).now().sign().await?;
                signed.call().await?
            };
            Ok(resp)
        }

        /// Spot trading
        pub fn spot(&self) -> &SpotApi<S> {
            SpotApi::ref_cast(&self)
        }
    }
}
