use std::ffi::c_void;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::time::{timeout, Duration};

use serde_json::Value as JsonValue;

use crate::bcos3sdk::bcos3sdkresponse::{bcos_sdk_c_struct_response, Bcos3SDKResponse};
use crate::bcossdkutil::kisserror::{KissErrKind, KissError};
use crate::kisserrcode;

#[repr(C)]
#[derive(Debug)]
pub struct Bcos3SDKFuture {
    pub seq: u64,
    pub name: String,
    pub desc: String,
    pub timeout: u64, // 等待返回的超时，默认为5秒
    pub tx: Sender<Bcos3SDKResponse>,
    pub rx: Receiver<Bcos3SDKResponse>,
}

static gSeq: AtomicU64 = AtomicU64::new(0);

impl Bcos3SDKFuture {
    pub fn next_seq() -> u64 {
        gSeq.fetch_add(1, Ordering::Relaxed)
    }

    // 使用 tokio::sync::mpsc::channel 实现异步回调的等待
    pub fn create(seq: u64, name: &str, desc: &str) -> Self {
        let (tx, rx) = mpsc::channel(1); // 异步通道，容量为1

        Bcos3SDKFuture {
            seq,
            name: name.to_string(),
            desc: desc.to_string(),
            timeout: 5,
            tx,
            rx,
        }
    }

    // 指针转换逻辑保持不变
    pub fn to_c_ptr(&self) -> *const c_void {
        self as *const Bcos3SDKFuture as *const c_void
    }

    pub fn from_c_ptr(ptr: *const c_void) -> *const Bcos3SDKFuture {
        ptr as *const Bcos3SDKFuture
    }

    pub fn display(&self) {
        println!(
            ">>>> context data:{}, [{}], [{}]",
            self.seq, self.name, self.desc
        );
    }

    // C 回调函数，保持不变
    pub extern "C" fn bcos_callback(resp: *const bcos_sdk_c_struct_response) {
        unsafe {
            let response = Bcos3SDKResponse::from_callback(resp);
            if !(*resp).context.is_null() {
                // 将指针转换为 Bcos3SDKFuture 并触发异步发送
                let future = Bcos3SDKFuture::from_c_ptr((*resp).context);
                (*future).fire(&response);
            }
        }
    }

    // 异步发送响应
    pub async fn fire(&self, resp: &Bcos3SDKResponse) {
        let _ = self.tx.send(resp.clone()).await;
    }

    // 异步等待响应
    pub async fn wait(&mut self) -> Result<Bcos3SDKResponse, KissError> {
        match timeout(Duration::from_secs(self.timeout), self.rx.recv()).await {
            Ok(Some(response)) => Ok(response),
            Ok(None) => Err(kisserrcode!(
                KissErrKind::EChannelClosed,
                -1,
                "channel closed"
            )),
            Err(_) => Err(kisserrcode!(KissErrKind::ETimeout, -1, "timeout")),
        }
    }

    // 异步等待并获取结果
    pub async fn wait_result(&mut self) -> Result<JsonValue, KissError> {
        let response = self.wait().await?;
        response.get_result()
    }
}
