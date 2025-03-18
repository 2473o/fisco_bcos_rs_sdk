#![allow(
    clippy::unreadable_literal,
    clippy::upper_case_acronyms,
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    overflowing_literals,
    unused_imports,
    unused_results,
    unused_variables
)]
use crate::common::Cli;
use rust_gears_sdk::bcos2sdk::bcos2client::Bcos2Client;
use rust_gears_sdk::bcos2sdk::bcossdkquery::json_hextoint;
use rust_gears_sdk::bcossdkutil::contractabi::ContractABI;
use rust_gears_sdk::bcossdkutil::contracthistory::ContractHistory;
use rust_gears_sdk::bcossdkutil::fileutils;
use rust_gears_sdk::bcossdkutil::kisserror::KissError;
use serde_json::{Value as JsonValue, json};
use std::thread;
use std::time::Duration;

//---------------------------------------------------------
pub fn demo(cli: &Cli) -> Result<(), KissError> {
    let mut bcossdk = Bcos2Client::new_from_config(cli.default_configfile().as_str()).unwrap();
    let res = bcossdk.queryGroupStatus(1)?;
    println!("querygroupstatus {:?}", res);
    Ok(())
}
