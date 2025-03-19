use rust_gears_sdk::bcossdkutil::accountutil::{load_key_from_pem, save_key_to_pem, EcdsaAccountUtil, IBcosAccountUtil};


//测试代码开始--------------------------------------------
#[test]
pub fn test_account() {
    let fixkey = "82dcd33c98a23d5d06f9331554e14ab4044a1d71b169b7a38b61c214f0690f80";
    //let account = EcdsaAccount::creat_random();
    let accountresult =
        EcdsaAccountUtil::default().from_privkey_bytes(&hex::decode(String::from(fixkey)).unwrap());
    let account = accountresult.unwrap();
    println!("account : {:?}", account);
    let pemfile = "sdk/test.pem";
    let _ = save_key_to_pem(&account.privkey, pemfile);
    let loadres = load_key_from_pem(pemfile);
    let accountload = EcdsaAccountUtil::default().from_privkey_bytes(&loadres.unwrap());
    println!("load result {:?}", accountload);
    println!("account in hex : {:?}", account.to_hexdetail());
}
