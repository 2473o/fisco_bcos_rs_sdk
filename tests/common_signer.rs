
static demokeyhex: &str = "82dcd33c98a23d5d06f9331554e14ab4044a1d71b169b7a38b61c214f0690f80";

pub fn test_common_sign() {
    //let mut ecdsasigner: CommonSignerSecp256 = CommonSignerSecp256::default();
    let mut wedprsigner: CommonSignerWeDPR_Secp256 = CommonSignerWeDPR_Secp256::default();
    let data = keccak_hash::keccak(Vec::from("abcdefg"));
    // ecdsasigner.from_hex_key(demokeyhex);
    wedprsigner.key_from_hexstr(demokeyhex);

    // let mut signer: &dyn ICommonSigner = &ecdsasigner;
    // let s1 = signer.sign(Vec::from(data.as_bytes())).unwrap();
    let signer = &wedprsigner;
    let s2 = signer.sign(Vec::from(data.as_bytes())).unwrap();
    //wedpr转公钥使用了带压缩支持的算法，前面加04是为了标注这个公钥是没有压缩的，64字节的公钥，如果是压缩的33字节公钥前面会是03
    let recover = wedprsigner
        .signer
        .recover_public_key(data.as_bytes(), s2.to_vec().as_slice())
        .unwrap();
    println!(
        "recover by wedpr ,pubkey len{},{:?}",
        &recover.len(),
        &recover
    );
    let sp = Secp256Signature::to_electrum(&s2.to_vec());
    /*
     let sig = ParityEcdsaSignature::from_electrum(sp.as_slice());
     let recoverresult = publickey::recover(&sig, &data).unwrap();
     println!(
     "recover by ecdsa ,pubkey len {}, {:?}",
     recoverresult.as_bytes().len(),
     recoverresult.as_bytes()
     );*/

    let s = CommonSignature::from_vec(&s2.to_vec());

    println!("r={:?},s={:?},v={:?}", s.r, s.s, s.v);
}

pub fn test_gm_sign() {
    let mut sm2signer = CommonSignerWeDPR_SM2::default();
    sm2signer.key_from_hexstr(demokeyhex);

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
