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

#[cfg(not(feature = "no_2sdk"))]
pub mod bcos2sdk;

#[cfg(feature = "bcos3sdk_ffi")]
pub mod bcos3sdk;
pub mod bcossdkutil;
