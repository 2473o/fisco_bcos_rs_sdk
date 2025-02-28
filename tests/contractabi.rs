use rust_gears_sdk::bcossdkutil::{commonhash::HashType, contractabi::ContractABI};
use hex_literal::hex;

pub fn test_parse_log() {
    let abi_path = "contracts/HelloWorld.abi";
    let contract_result = ContractABI::new(abi_path, &HashType::WEDPR_KECCAK);
    let logdata = "000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000103132333437383930616263656667686500000000000000000000000000000000";
    let contract = contract_result.unwrap();
    for (pos, e) in contract.contract.events.iter().enumerate() {
        println!("event {:?}", e);
    }
    let onset_events = contract.contract.events_by_name("onset").unwrap();

    for (pos, e) in onset_events.iter().enumerate() {
        println!("Element at position {}: {:?}", pos, e);
        println!(
            "event signature(topic) {:?}",
            hex::encode(e.signature().as_bytes())
        );
        let rawlog = ethabi::RawLog {
            topics: vec![hex!("afb180742c1292ea5d67c4f6d51283ecb11e49f8389f4539bef82135d689e118").into()],
            data: hex!("000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000103132333437383930616263656667686500000000000000000000000000000000")
                .into(),
        };
        println!("{:?}", rawlog);
        let ebyhash = contract.find_event_by_hashstring(String::from(
            "afb180742c1292ea5d67c4f6d51283ecb11e49f8389f4539bef82135d689e118",
        ));
        match ebyhash {
            Some(event) => {
                let result = e.parse_log(rawlog.clone());
                let log = result.ok().unwrap();
                println!("log  by  hash is : {:?}", log);
            }
            None => {
                println!("not fond event by hash");
            }
        }

        let result = e.parse_log(rawlog);
        let log = result.ok().unwrap();
        println!("log is : {:?}", log);
    }
}

pub fn test_contract() {
    let abi_path = "contracts/HelloWorld.abi";
    let contract = ContractABI::new(abi_path, &HashType::WEDPR_KECCAK);
    match &contract {
        Ok(c) => {
            println!("contract is {:?}", c);
        }
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    }
    let params: [String; 1] = [String::from("12347890abc")];
    let hellores = contract
        .unwrap()
        .encode_function_input_to_abi("set", &params, false)
        .ok();
    println!("contract  set rawdata :{}", hellores.unwrap().as_str());
    test_parse_log();
    test_parse_tx_input();
}

pub fn test_parse_tx_input() {
    let txinput = "4ed3885e000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000103132333437383930616263656667686500000000000000000000000000000000";
    let abi_path = "contracts/HelloWorld.abi";
    let contract = ContractABI::new(abi_path, &HashType::WEDPR_KECCAK);
    println!("{:?}", &contract);
    let funopt = contract.unwrap().decode_input_for_tx(txinput);
    match funopt {
        Ok(input_result) => {
            println!("{:?}", input_result);
            println!("function is {:?}", input_result.func);
            let parseresult = &input_result.input;
            println!("parseresult : {:?}", parseresult);
            for t in parseresult.iter() {
                println!("{}", input_result.func.name);
                println!("{}", t.to_string());
            }
        }
        Err(e) => {
            println!("not found func {:?}", e);
        }
    }
}
