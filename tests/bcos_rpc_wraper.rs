#[cfg(not(feature = "no_2sdk"))]
use rust_gears_sdk::{
    bcos2sdk::bcosrpcwraper::BcosRPC, bcossdkutil::bcosclientconfig::ClientConfig,
};

#[cfg(not(feature = "no_2sdk"))]
#[test]
pub fn test_json_rpc() {
    let groupid = 1;
    let config = ClientConfig::load("conf/client_config.toml").unwrap();
    let mut client = BcosRPC::new(&config).unwrap();
    let params = &json!([groupid]);
    let response = client.rpc_request_sync("getBlockNumber", params);
    println!("{:?}", response);
}
