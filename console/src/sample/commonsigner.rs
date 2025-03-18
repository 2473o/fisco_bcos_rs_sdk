use rust_gears_sdk::bcossdkutil::commonsigner::{CommonSignerWeDPR_SM2, ICommonSigner};
use wedpr_l_libsm::sm2::signature::Signature as WEDPRSM2Signature;

static DEMO_KEY_HEX: &str = "82dcd33c98a23d5d06f9331554e14ab4044a1d71b169b7a38b61c214f0690f80";

pub fn test_gm_sign() {
    let mut sm2signer = CommonSignerWeDPR_SM2::default();
    sm2signer.key_from_hexstr(DEMO_KEY_HEX);

    let signer: &dyn ICommonSigner = &sm2signer;
    let data = "1234567890";
    let signresult = signer.sign(data.as_bytes().to_vec());
    println!("GM Sign result = {:?}", &signresult);
    let signresult1 = signer.sign(data.as_bytes().to_vec());
    let sig = signresult.unwrap();
    println!("account detail: {:?}", sm2signer.account.to_hexdetail());
    println!("GM Sign Hex = {:?}", hex::encode(&sig.to_vec().as_slice()));

    let sigsm2 = WEDPRSM2Signature::bytes_decode(&sig.to_vec().as_slice()).unwrap();
    println!("sm2 sig {:?}", sigsm2);
    println!(
        "sm sig is r:{:?},s{:?},v:{:?}({})",
        hex::encode(&sig.r),
        hex::encode(&sig.s),
        hex::encode(&sig.v),
        &sig.v.len()
    );
}
