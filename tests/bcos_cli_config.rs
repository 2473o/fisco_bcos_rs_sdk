use rust_gears_sdk::bcossdkutil::bcosclientconfig::ClientConfig;


//------------------------------------------------------------------------
#[test]
pub fn test_config() {
    let res = ClientConfig::load("conf/client_config.toml");
    println!("{:?}", res);
}
