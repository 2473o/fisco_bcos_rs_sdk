use rust_gears_sdk::bcos2sdk::channelpack::ChannelPack;

pub fn test_channelpack() {
    let mut pack = ChannelPack::default();
    let data = "1234567890";
    pack.data = Vec::from(data);
    pack.seq = ChannelPack::make_seq();
    pack.packtype = 0x12;
    pack.result = 0;
    pack.length = 42 + pack.data.len();
    let bin = pack.pack();
    //let bin  = serialize(&pack).unwrap();
    println!("{:?}", &bin);
    let hexv = hex::encode(bin.as_slice());
    println!("{:?}", &hexv);
    println!("totallen = {}", hexv.len());

    let unpackres = ChannelPack::unpack(&bin).unwrap();
    if unpackres.seq != pack.seq {
        println!("ne");
    } else {
        println!("seq eq");
    }
    println!("unpack: {:?}", unpackres);
    println!("data content {}", String::from_utf8(pack.data).unwrap());
}
/*
https://fisco-bcos-documentation.readthedocs.io/zh_CN/latest/docs/design/protocol_description.html#id5

0x12	JSONRPC 2.0格式	RPC接口消息包	SDK->节点
0x13	json格式心跳包{"heartbeat":"0"}	心跳包	0:SDK->节点，1:节点->SDK
0x14	SDK->节点的包体{"minimumSupport":version,"maximumSupport":version,"clientType":"client type"},节点->SDK的包体{"protocol":version,"nodeVersion":"fisco-bcos version"	握手包，json格式的协议版本协商	SDK<->节点，双向
0x30	AMOP消息包包体	AMOP请求包	SDK<->节点，双向
0x31	失败的AMOP消息的包体	AMOP失败响应包	节点->SDK或节点->节点
0x32	json数组，存储SDK监听的Topics	上报Topic信息	SDK->节点
0x35	AMOP消息包包体	AMOP多播消息	节点->节点
0x1000	json格式的交易上链通知	交易上链回调	节点->SDK
0x1001	json格式的区块上链通知{"groupID":"groupID","blockNumber":"blockNumber"}
*/
/*
https://fisco-bcos-documentation.readthedocs.io/zh_CN/latest/docs/design/protocol_description.html#id6
错误码
code	message
0	成功
100	节点不可达
101	SDK不可达
102	超时
*/
