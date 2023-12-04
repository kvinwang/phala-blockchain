use scale::{Decode, Encode};

pub type AccountId = [u8; 32];
pub type H256 = [u8; 32];

#[derive(Encode, Decode)]
pub struct QueryRequest {
    pub origin: Option<AccountId>,
    pub payload: Vec<u8>,
    pub reply_tx: i32,
}

#[derive(Encode, Decode, Debug)]
pub struct HttpHead {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
}

impl HttpHead {
    pub fn get_header(&self, key: &str) -> Option<&str> {
        let key = key.to_ascii_lowercase();
        for (k, v) in &self.headers {
            if k.to_ascii_lowercase() == key {
                return Some(v);
            }
        }
        None
    }
}

#[derive(Encode, Decode, Debug)]
pub struct HttpRequest {
    pub head: HttpHead,
    pub response_tx: i32,
    pub io_stream: i32,
}

#[derive(Encode, Decode, Debug)]
pub struct HttpResponseHead {
    pub status: u16,
    pub headers: Vec<(String, String)>,
}

#[derive(Encode, Decode)]
pub enum SystemMessage {
    PinkLog {
        block_number: u32,
        contract: AccountId,
        entry: AccountId,
        exec_mode: String,
        timestamp_ms: u64,
        level: u8,
        message: String,
    },
    PinkEvent {
        block_number: u32,
        contract: AccountId,
        topics: Vec<H256>,
        payload: Vec<u8>,
    },
    PinkMessageOutput {
        block_number: u32,
        origin: AccountId,
        contract: AccountId,
        nonce: Vec<u8>,
        output: Vec<u8>,
    },
    Metric(Metric),
}

#[derive(Encode, Decode)]
pub enum Metric {
    PinkQueryIn([u8; 8]),
}
