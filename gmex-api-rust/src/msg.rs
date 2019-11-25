//

use md5;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/** 进行http和交易服务器通讯时的请求消息体 */
#[derive(Debug, Serialize, Deserialize)]
pub struct HttpTradeRequestMessage<T> {
    req: String,
    args: T,
    expires: i64,
    username: String,
    apikey: String,
    signature: String,
}

impl<T> HttpTradeRequestMessage<T>
where
    T: Serialize,
{
    pub fn new(req: &str, args: T, uname: &String, api_key: &String, api_secret: &String) -> Self {
        let now_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        let expires = (now_ms as i64) + 5000; // 设置5秒过期, FIXME!!!
        let s1 = serde_json::to_string(&args).unwrap();
        // 签名计算公式: MD5(req+args+expires+API.SecretKey)
        let digest = md5::compute(format!("{}{}{}{}", req, s1, expires, api_secret));
        Self {
            req: req.to_string(),
            args: args,
            expires: expires,
            username: uname.into(),
            apikey: api_key.into(),
            signature: format!("{:x}", digest),
        }
    }
}

/** 服务器端http请求的返回结果消息体 */
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HttpResponseMessage<T> {
    // 出错时，错误码和错误消息文本描述
    ErrData { code: i32, data: Option<String> },
    // 对应请求返回的结果的消息体
    RspData { code: i32, data: Option<T> },
}

impl<T> HttpResponseMessage<T> {
    pub fn has_error(&self) -> bool {
        match self {
            Self::ErrData { code, data: _ } => *code != 0,
            Self::RspData { code, data: _ } => *code != 0,
        }
    }

    pub fn get_error_msg(&self) -> Option<&String> {
        match self {
            Self::ErrData { code: _, data } => data.as_ref(),
            Self::RspData { code: _, data: _ } => None,
        }
    }

    pub fn get_code(&self) -> i32 {
        match *self {
            Self::ErrData { code, data: _ } => code,
            Self::RspData { code, data: _ } => code,
        }
    }
    pub fn get_rsp_data(&self) -> Option<&T> {
        match self {
            Self::ErrData { code: _, data: _ } => None,
            Self::RspData { code: _, data } => data.as_ref(),
        }
    }

    pub fn take_rsp_data(&mut self) -> Option<T> {
        match self {
            Self::RspData { code: _, data } => data.take(),
            _ => None,
        }
    }
}

/** 和服务器ws通信时收到的服务器端过来的消息体 */
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WsResponseMessage<T> {
    // 出错时，错误码和错误消息文本描述
    ErrData {
        code: i32,
        rid: Option<String>,
        data: Option<String>,
    },
    // 对应请求返回的结果的消息体
    RspData {
        code: i32,
        rid: Option<String>,
        data: Option<T>,
    },
    // 服务器端主动推送过来的消息体
    PushData {
        subj: Option<String>,
        data: Option<T>,
    },
}
