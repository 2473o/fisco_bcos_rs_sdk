use std::thread;

use rust_gears_sdk::{bcos2sdk::{bcos_channel_client::IBcosChannel, bcos_ssl_native::{getNodeVersionPack, BcosNativeTlsClient}, channelpack::ChannelPack}, bcossdkutil::bcosclientconfig::ClientConfig};
use std::time::Duration;

pub fn test_ssl_native() {
    let config = ClientConfig::load("gm/conf/config.toml").unwrap();
    let mut client = BcosNativeTlsClient::default(&config.channel);
    let res = client.build();
    println!("client build result {:?}", res);
    println!("Client sock is {:?}", client.ssocklib);
    //let res = client.connect();
    //println!("connect result = {:?}",res);
    let buffer = getNodeVersionPack().unwrap().pack();
    let sendres = client.send(&buffer);
    println!("send result  = {:?}", sendres);
    loop {
        //let dstlen = 1024;
        let recvres = client.recv();
        //println!("recv result {:?}", r);
        if recvres.is_ok() {
            let recvbuffer = recvres.unwrap().clone();

            if recvbuffer.len() > 0 {
                println!("{:?}", recvbuffer);
                let p = ChannelPack::unpack(&recvbuffer).unwrap();
                println!("pack: {}", p.detail());
                println!("data: {}", String::from_utf8(p.data).unwrap());

                break;

            }

        }
        thread::sleep(Duration::from_millis(300));
    }
    client.finish();
}
