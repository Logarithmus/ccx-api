use ccx_api_lib::GateApiCred;
use ccx_gate::GateApi;

#[actix_rt::main]
async fn main() {
    let _ = dotenvy::dotenv();

    env_logger::init();

    let api = GateApi::<GateApiCred>::from_env();

    dbg!(api.spot().tickers(&Default::default()).await).unwrap();
}
