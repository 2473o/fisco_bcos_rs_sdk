
//------------------------------------------------------------------------
pub fn test_config() {
    let res = ClientConfig::load("conf/client_config.toml");
    println!("{:?}", res);
}
