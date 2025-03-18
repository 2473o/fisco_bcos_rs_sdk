use std::collections::HashMap;

use rust_gears_sdk::bcossdkutil::contracthistory::ContractHistory;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct GroupData {
    pub name: HashMap<String, HashMap<String, String>>,
}

pub fn test_multi() {
    //let mut gd:HashMap<String,HashMap<String,String>> = HashMap::new();
    let mut gd = GroupData {
        name: HashMap::new(),
    };
    let mut gd1: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut car: HashMap<String, String> = HashMap::new();
    car.insert("c260".to_string(), "benz".to_string());
    car.insert("a3".to_string(), "audi".to_string());
    car.insert("x3".to_string(), "bmw".to_string());

    let mut fruit: HashMap<String, String> = HashMap::new();
    fruit.insert("apple".to_string(), "red".to_string());
    fruit.insert("banana".to_string(), "yellow".to_string());

    gd.name.insert("carbrand".to_string(), car.clone());
    gd.name.insert("fruitcolor".to_string(), fruit.clone());

    gd1.insert("carbrand".to_string(), car.clone());
    gd1.insert("fruitcolor".to_string(), fruit.clone());

    let res = toml::to_string_pretty(&gd1).unwrap();
    println!("{}", res);

    let mut v: HashMap<String, HashMap<String, String>> = toml::from_str(res.as_str()).unwrap();
    println!("from toml:{:?}", v);
    let mut car = v.get("carbrand").unwrap().clone();
    car.insert("x3".to_string(), "bwm_2022".to_string());
    v.insert("carbrand".to_string(), car.clone());
    println!("after change {}", toml::to_string_pretty(&v).unwrap());
}

pub fn test_toml() {
    //return test_multi();

    let history_name = "contracts/contracthistory1.toml";
    let mut ch = ContractHistory::load(history_name).unwrap();
    println!("{:?}", ch);
    let addr = ch.getlast("seg2", "HelloWorld").unwrap();
    println!("get by name {}", addr);
    ch.add("seg1", "a", "0xabcdefg", 0);
    ch.add("seg2", "HelloWorld1", "0xef1234567890", 99);
    let res = ch.save(history_name);

    println!("getlast {:?}", ch.getlast("seg2", "HelloWorld"));
    println!(
        "find by address {:?}",
        ch.find_record_by_address("seg2", "0x02687d278477a84328446e580f79cfb12ab219e4")
    );
}
