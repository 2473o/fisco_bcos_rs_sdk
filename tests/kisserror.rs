use failure::AsFail;
use rust_gears_sdk::{
    bcossdkutil::kisserror::{KissErrKind, KissError},
    kisserr,
};

pub fn test_fire_error(i: u32) -> Result<String, KissError> {
    if i > 10 {
        Ok("ok done".to_string())
    } else {
        kisserr!(KissErrKind::ENetwork, "")
    }
}
pub fn test_enum_error() -> Result<String, KissErrKind> {
    Err(KissErrKind::EArgument)
}
pub fn test_bcos_error() {
    let r = test_fire_error(10);
    match r {
        Ok(v) => {
            println!("{:?}", v);
        }
        Err(e) => {
            println!("{:?}", e.kind.as_fail().cause());
            println!("{:?}", e);
        }
    }

    let rr = test_enum_error();
    match rr {
        Err(e) => {
            println!("{:?}", e.as_fail().cause());
        }
        _ => {}
    }
}
