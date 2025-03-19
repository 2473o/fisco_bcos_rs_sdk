
use crate::bcos2sdk::bcos2_ssl_ffi::*;

use crate::bcos2sdk::bcosrpcwraper::RpcRequestData;

use crate::bcos2sdk::channelpack::{make_channel_pack, ChannelPack, CHANNEL_PACK_TYPE};

use crate::str2p;

use libc::{c_char, c_int};

use serde_json::json;

use std::ffi::CString;
use std::thread;
use std::time::Duration;

//use std::ffi::CStr;
//use libc::size_t;
//ffi 模式的调用，需要native_ssock_wrap.lib文件
//打开tassl_sock_ffi特性，需要用这个语句编译：cargo build --features  "tassl_sock_ffi"


pub fn getNodeVersionPack() -> Option<ChannelPack> {
    let groupid = 1;
    let cmd = "getClientVersion";
    let params_value = json!([groupid]);

    let req = RpcRequestData {
        method: cmd.to_string(),
        params: params_value.clone(),
        jsonrpc: "2.0".to_string(),
        id: 1,
    };
    println!("{:?}", req);
    make_channel_pack(CHANNEL_PACK_TYPE::RPC, req.encode().unwrap().as_str())
}

#[cfg(feature = "libtestcallback")]
pub extern "C"  fn fn_callback(buffer: *mut c_char, buffersize: c_int) -> c_int {
    use std::ffi::CStr;

    println!("IN CALLBACK {}", buffersize);
    println!("IN CALLBACK {:?}", buffer);
    unsafe {
        let cs = CStr::from_ptr(buffer.clone());
        println!("cs : {:?}", &cs);
        let content = "1024 from rust";
        buffer.copy_from(content.as_ptr() as *const c_char, content.len());

        return content.len() as c_int;
    }
}
