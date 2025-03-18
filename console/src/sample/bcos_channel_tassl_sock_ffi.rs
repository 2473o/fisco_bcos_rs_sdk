use std::{ffi::CString, thread, time::Duration};

use libc::{c_char, c_int};
use rust_gears_sdk::bcos2sdk::bcos_channel_tassl_sock_ffi::fn_callback;
use rust_gears_sdk::{
    bcos2sdk::{
        bcos_channel_tassl_sock_ffi::getNodeVersionPack,
        bcos2_ssl_ffi::{
            ssock_create, ssock_finish, ssock_init, ssock_recv, ssock_send, ssock_set_echo_mode,
            ssock_try_connect,
        },
        channelpack::ChannelPack,
    },
    str2p,
};

#[cfg(feature = "libtestcallback")]
pub fn test_callback() {
    use std::{ffi::c_void, thread, time::Duration};

    #[link(name = "libtestcallback")]
    unsafe extern "C" {
        unsafe fn dotest(fncb: *mut c_void);
    }
    unsafe {
        dotest(fn_callback as *mut c_void);
        {
            thread::sleep(Duration::from_secs(5));
        }
    }
}

pub fn test_ssock() {
    println!("test ssock ffi");
    #[cfg(feature = "bcos2sdk_ffi")]
    unsafe {
        /* let cafile = CString::new("gm/sdk/gmca.crt").unwrap();
        let sdkcrt = CString::new("gm/sdk/gmsdk.crt").unwrap();
        let sdkkey = CString::new("gm/sdk/gmsdk.key").unwrap();
        let ensdk = CString::new("gm/sdk/gmensdk.crt").unwrap();
        let ensdkkey = CString::new("gm/sdk/gmensdk.key").unwrap();*/
        let cafile = "gm/sdk/gmca.crt";
        let sdkcrt = "gm/sdk/gmsdk.crt";
        let sdkkey = "gm/sdk/gmsdk.key";
        let ensdk = "gm/sdk/gmensdk.crt";
        let ensdkkey = "gm/sdk/gmensdk.key";

        let psock = ssock_create();
        println!("{:?}", psock);
        ssock_set_echo_mode(psock, 1 as c_int);
        ssock_init(
            psock,
            //cafile.as_ptr(),
            str2p!(cafile),
            str2p!(sdkcrt),
            str2p!(sdkkey),
            str2p!(ensdk),
            str2p!(ensdkkey),
        );
        let ip = CString::new("119.29.114.153").unwrap();
        let res = ssock_try_connect(psock, ip.as_ptr(), 20800);
        println!("connnect result {}", res);
        let pack = getNodeVersionPack();
        let reqdata = pack.unwrap().pack();
        let res = ssock_send(
            psock,
            reqdata.as_ptr() as *mut c_char,
            reqdata.len() as c_int,
        );
        let size: usize = 1024 * 10;
        let mut recvbuffer: Vec<u8> = Vec::with_capacity(size);
        let pdst = recvbuffer.as_mut_ptr();
        loop {
            //let dstlen = 1024;
            let r = ssock_recv(
                //self.ssocklib.as_ref().unwrap().pssock,
                psock,
                pdst as *mut c_char,
                size as i32,
            );
            //println!("recv result {:?}", r);
            if r > 0 {
                //println!("recv size :{}",r);
                println!("r = {}", r);
                recvbuffer.set_len(r as usize);

                println!("{:?}", recvbuffer);
                let p = ChannelPack::unpack(&recvbuffer).unwrap();
                println!("pack(FFI): {}", p.detail());
                break;
            } else {
                thread::sleep(Duration::from_millis(100));
            }
        }
        ssock_finish(psock);
    }
}
