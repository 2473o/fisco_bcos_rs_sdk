/*
  FISCO BCOS/rust-SDK is a rust client for FISCO BCOS2.0 (https://github.com/FISCO-BCOS/)
  FISCO BCOS/rust-SDK is free software: you can redistribute it and/or modify it under the
  terms of the MIT License as published by the Free Software Foundation. This project is
  distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even
  the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
  @author: kentzhang
  @date: 2021-07
*/
#![allow(
    clippy::unreadable_literal,
    clippy::upper_case_acronyms,
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    overflowing_literals,
    unused_variables,
    unused_assignments
)]
use thiserror::Error;

#[macro_export]
macro_rules! kisserr {
            ($x:expr,$($arg:tt)*) => {
                Err(KissError::new(
                    ($x),
                    -1,
                    format!($($arg)*).as_str()
                ))
            };
}

#[macro_export]
macro_rules! kisserrcode {
            ($x:expr,$code:expr,$($arg:tt)*) => {
                Err(KissError::new(
                    ($x),
                    ($code),
                    format!($($arg)*).as_str()
                ))
            };
}

//Kiss: Keep It Simple & Stupid
#[derive(Error, Clone, Debug, Eq, PartialEq)]
pub enum KissErrKind {
    #[error("error")]
    Error,
    #[error("Argument is invalid")]
    EArgument,
    #[error("Data format error")]
    EFormat,
    #[error("net work error")]
    ENetwork,
    #[error("timeout")]
    ETimeout,
    #[error("sign fail")]
    ESign,
    #[error("try again")]
    EAgain,
    #[error("file not exist")]
    EFileMiss,
    #[error("file open")]
    EFileOpen,
    #[error("file write")]
    EFileWrite,
    #[error("file read")]
    EFileRead,
}

impl Default for KissErrKind {
    fn default() -> Self {
        KissErrKind::Error
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KissError {
    pub kind: KissErrKind,
    pub code: i64,
    pub msg: String,
}

impl KissError {
    pub fn err(kind: KissErrKind) -> KissError {
        KissError {
            kind: kind,
            code: 0,
            msg: "".to_string(),
        }
    }
    pub fn new(kind: KissErrKind, code: i64, msg: &str) -> KissError {
        KissError {
            kind: kind,
            code: code,
            msg: msg.to_string(),
        }
    }
}
