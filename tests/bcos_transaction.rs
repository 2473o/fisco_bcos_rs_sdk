use ethereum_types::{Address, H256, H512, U256};
use rust_gears_sdk::{
    bcos2sdk::bcostransaction::{encode_raw_transaction, BcosTransaction, BcosTransactionWithSig},
    bcossdkutil::{
        accountutil::{EcdsaAccountUtil, IBcosAccountUtil},
        commonhash::HashType,
    },
};
use std::str::FromStr;

fn test_decode_tx_from_str(tx_data: &str) {
    let tx = BcosTransaction::decode_bytes(hex::decode(tx_data).unwrap().as_slice()).unwrap();
}

///测试代码入口
pub fn test_sign_tx() {
    let key = EcdsaAccountUtil::default().create_random();
    let set_input = "4ed3885e0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000b3132333437383930616263000000000000000000000000000000000000000000";
    //let addr = Address::from_str("40034be5fd46006238c04c2cedfe92dbddbdb651").unwrap();
    //let addr = String::from("");
    let addr = String::from("40034be5fd46006238c04c2cedfe92dbddbdb651");
    let raw_tx_encode = encode_raw_transaction(
        &addr,
        &String::from(set_input),
        &key.privkey,
        HashType::WEDPR_KECCAK,
    );
    println!("raw_tx_encode {:?}", raw_tx_encode);
    let tx = BcosTransactionWithSig::decode_bytes(&raw_tx_encode).unwrap();
    println!("after decode SignedBcosTransaction");
    println!(
        "to address {:?}",
        Address::from_slice(tx.transaction.to_address.as_slice())
    );
    println!("to address {:?}", hex::encode(tx.transaction.to_address));
}

#[test]
pub fn test_decode_tx() {
    //let datahex: &str = "f864808504a817c800825208943535353535353535353535353535353535353535808025a0044852b2a670ade5407e78fb2863c51de9fcb96542a07186fe3aeda6bb8a116da0044852b2a670ade5407e78fb2863c51de9fcb96542a07186fe3aeda6bb8a116d";
    let datahex = "f8ea2a820bb882c3503294d46e8dd67c5d32be8058bb8eb970870f0724456701b8c834656433383835653030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030323030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303034333133323333333430303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030300101836162631ca0ecf1aa38a05271a7d0c90a6f85b558458620812244d04bebbe94e093a45404faa05e4b294198175184cb4b35d109d00457300afefee5efc3cadf59718bc27bfd47";
    let rawdata = hex::encode("abcdefg");
    let randid = 209;

    let tx = BcosTransaction {
        to_address: Vec::from(
            Address::from_str("40034be5fd46006238c04c2cedfe92dbddbdb651")
                .unwrap()
                .as_bytes(),
        ),
        random_id: U256::from(randid),
        gas_price: U256::from(30000000),
        gas_limit: U256::from(30000000),
        block_limit: U256::from(501),
        value: U256::from(0),
        data: hex::decode(rawdata.as_str()).unwrap(),
        fisco_chain_id: U256::from(1),
        group_id: U256::from(1),
        extra_data: b"".to_vec(),
        hashtype: HashType::WEDPR_KECCAK,
    };
    let txencodedata = tx.encode();
    //println!("hash tx {:?}",tx.hash());
    //println!("tx hex: {:?}",&hex::encode(datahex.as_slice()));
    //let hexstr = hex::encode(datahex.as_slice()).as_str();
    //test_decode_tx_from_str(txencodedata.as_slice().to_hex().as_str());

    test_sign_tx();
}
