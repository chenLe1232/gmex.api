use std::collections::HashMap;
use uuid::Uuid;

use log;
use md5;
use ws;

#[macro_use]
extern crate failure;
use failure::Error;

#[macro_use]
extern crate serde_json;
use serde::{Deserialize, Serialize};

use gmex_api;

// use std::sync::mpsc::channel;
// use std::sync::mpsc::Sender as ThreadOut;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
struct WsResponseMessage {
    // 如果有 rid，则说明是request的返回结果消息
    rid: Option<String>,
    // 操作结果，0表示成功，其它表示失败, 失败时data里通常是个string说明原因
    code: Option<i32>,
    // 如果有 subj，则说明是推送过来的消息
    subj: Option<String>,
    // 结果信息, 根据请求对应返回结果，具体结构定义根据请求而来，请参考API文档对应
    data: Option<serde_json::Value>,
}

fn websocket_market_demo(market_base_url: &String) -> Result<(), Error> {
    type WsMsgCallBack = Option<Box<dyn FnMut(&mut Client, &mut WsResponseMessage)>>;

    struct Client {
        out: ws::Sender,
        callbacks: HashMap<String, WsMsgCallBack>,
    }
    impl Client {
        pub fn new(out: ws::Sender) -> Self {
            Client {
                out,
                callbacks: HashMap::new(),
            }
        }

        pub fn do_request<T: Serialize>(&mut self, req: &str, args: T, cb: WsMsgCallBack) -> ws::Result<()> {
            let rid = Uuid::new_v4().to_simple().to_string();
            let expires = gmex_api::time_now_msec() + 5000; // 设置5秒过期, FIXME!!!
            let msg = json!({
                "req": req,
                "rid": rid,
                "args": args,
                "expires": expires,
            });
            let buf = serde_json::to_vec(&msg).map_err(|e| ws::Error::new(ws::ErrorKind::Custom(Box::new(e)), ""))?;

            if cb.is_some() {
                self.callbacks.insert(rid, cb);
            }
            self.out.send(buf)?;

            Ok(())
        }
    }
    impl ws::Handler for Client {
        fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
            log::debug!("[mkt_ws.on_open]");
            // self.do_request("GetAssetD", json!({}), None)?;
            // self.do_request("GetAssetEx", json!({}), None)?;
            self.do_request("GetCompositeIndex", json!({}), None)?;
            // self.do_request("Sub", json!(["tick_BTC.USDT", "order20_BTC.USDT", "kline_1m_BTC.USDT", "__slow__"]), None)?;
            self.do_request("Sub", json!(["tick_BTC.USDT", "__slow__"]), None)?;

            Ok(())
        }

        fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
            log::debug!("[mkt_ws.on_message] {}", msg);
            // self.out.close(ws::CloseCode::Normal)

            if msg.is_binary() {
                log::warn!("mkt_ws got binary response-msg, ignore now, 暂时不支持二进制消息, TODO!!!");
                return Ok(());
            }

            let body = msg.as_text()?;
            let mut resp: WsResponseMessage =
                serde_json::from_str(body).map_err(|e| ws::Error::new(ws::ErrorKind::Custom(Box::new(e)), ""))?;

            if resp.rid.is_some() {
                let rid = resp.rid.as_ref().unwrap();
                log::info!("mkt_ws got rid={} response: {}", rid, body);
                // let rid: i64 = rid.parse().unwrap_or_default();
                if let Some(cb) = self.callbacks.remove(rid) {
                    cb.unwrap()(self, &mut resp);
                }
            } else {
                match resp.subj.as_ref().map(|s| &s[..]) {
                    Some("tick") => {
                        let data: gmex_api::MktInstrumentTick =
                            serde_json::from_value(resp.data.unwrap_or_default())
                                .map_err(|e| ws::Error::new(ws::ErrorKind::Custom(Box::new(e)), ""))?;
                        log::info!("[mkt.tick]: {:?}", data);
                    }
                    Some("trade") => {
                        let data: gmex_api::MktTradeItem = serde_json::from_value(resp.data.unwrap_or_default())
                            .map_err(|e| ws::Error::new(ws::ErrorKind::Custom(Box::new(e)), ""))?;
                        log::info!("[mkt.trade]: {:?}", data);
                    }
                    Some("order20") => {
                        let data: gmex_api::MktOrder20Result = serde_json::from_value(resp.data.unwrap_or_default())
                            .map_err(|e| ws::Error::new(ws::ErrorKind::Custom(Box::new(e)), ""))?;
                        log::info!("[mkt.order20]: {:?}", data);
                    }
                    Some("orderl2") => {
                        let data: gmex_api::MktOrderItem = serde_json::from_value(resp.data.unwrap_or_default())
                            .map_err(|e| ws::Error::new(ws::ErrorKind::Custom(Box::new(e)), ""))?;
                        log::info!("[mkt.orderl2]: {:?}", data);
                    }
                    Some("kline") => {
                        let data: gmex_api::MktKLineItem = serde_json::from_value(resp.data.unwrap_or_default())
                            .map_err(|e| ws::Error::new(ws::ErrorKind::Custom(Box::new(e)), ""))?;
                        log::info!("[mkt.kline]: {:?}", data);
                    }
                    Some("index") => {
                        let data: gmex_api::MktCompositeIndexTick =
                            serde_json::from_value(resp.data.unwrap_or_default())
                                .map_err(|e| ws::Error::new(ws::ErrorKind::Custom(Box::new(e)), ""))?;
                        log::info!("[mkt.index]: {:?}", data);
                    }
                    Some("notification") => {
                        log::info!("[mkt.notification]: {:?}", body);
                    }
                    _ => {
                        log::warn!("[-] unknow mkt.resp msg body: {}", body);
                    }
                }
            }
            Ok(())
        }
    }

    ws::connect(market_base_url.as_str(), |out| {
        ws::deflate::DeflateBuilder::new().build(Client::new(out))
    })?;

    // Give the server a little time to get going
    sleep(Duration::from_millis(2000));

    // TODO
    Ok(())
}

fn websocket_trade_demo(
    trade_base_url: &String,
    user_name: &String,
    api_key: &String,
    api_secret: &String,
) -> Result<(), Error> {
    type WsMsgCallBack = Option<Box<dyn FnMut(&mut Client, &mut WsResponseMessage)>>;

    struct Client {
        out: ws::Sender,
        user_name: String,
        api_key: String,
        api_secret: String,
        callbacks: HashMap<String, WsMsgCallBack>,
        // login成功后得到用户的uid，否则为空
        uid: String,
    }

    impl Client {
        pub fn new(out: ws::Sender, user_name: &String, api_key: &String, api_secret: &String) -> Self {
            Client {
                out,
                user_name: user_name.to_owned(),
                api_key: api_key.to_owned(),
                api_secret: api_secret.to_owned(),
                callbacks: HashMap::new(),
                uid: String::new(),
            }
        }

        pub fn do_request<T: Serialize>(
            &mut self,
            req: &str,
            args: T,
            cb: WsMsgCallBack,
            // cb:  Box<dyn FnMut(WsResponseMessage) + 'a>
        ) -> ws::Result<()> {
            let rid = Uuid::new_v4().to_simple().to_string();
            let expires = gmex_api::time_now_msec() + 5000; // 设置5秒过期, FIXME!!!
            let s1 =
                serde_json::to_string(&args).map_err(|e| ws::Error::new(ws::ErrorKind::Custom(Box::new(e)), ""))?;
            // 生成签名的公式: md5(Req+rid+Args+Expires+API.SecretKey)
            let txt = format!("{}{}{}{}{}", req, rid, s1, expires, self.api_secret);
            let digest = md5::compute(txt);
            let msg = json!({
                "req": req,
                "rid": rid,
                "args": args,
                "expires": expires,
                "username": self.user_name,
                "apikey": self.api_key,
                "signature": format!("{:x}", digest),
            });
            let buf = serde_json::to_vec(&msg).map_err(|e| ws::Error::new(ws::ErrorKind::Custom(Box::new(e)), ""))?;
            if cb.is_some() {
                self.callbacks.insert(rid, cb);
            }
            self.out.send(buf)
        }
    }

    impl ws::Handler for Client {
        fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
            // log::debug!("[trd_ws.on_open]");
            log::debug!("[trd.ws] send login for {}", self.user_name);
            self.do_request(
                "Login",
                json!({"UserName": self.user_name, "UserCred": self.api_key}),
                Some(Box::new(move |cli, ret| {
                    // 收到login成功的消息或失败的消息
                    log::info!("GOT login response: {:?}", ret.data);
                    if ret.code.unwrap_or_default() == 0 && ret.data.is_some() {
                        // {"rid":"0","code":0,"data":{"UserName":"xxx@yyy.com","UserId":"1234567"}}
                        if let Some(info) = &ret.data {
                            // login ok, save uid HERE!!! update username if need.
                            cli.uid = info["UserId"].to_string();
                            cli.user_name = info["UserName"].to_string();
                            log::info!("Now set UID={}, and UserName={}", cli.uid, cli.user_name);
                        }
                    } else {
                        // {"rid":"0","code":-1,"data":"invalid args"}
                        log::warn!("[-] login failed: {:?}", ret);
                    }
                })),
            )
        }

        fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
            log::debug!("[trd]<< {}", msg);
            // self.out.close(ws::CloseCode::Normal)

            if msg.is_binary() {
                log::warn!("trd_ws got binary response-msg, ignore now, 暂时不支持二进制消息, TODO!!!");
                return Ok(());
            }

            let body = msg.as_text()?;
            let mut resp: WsResponseMessage =
                serde_json::from_str(body).map_err(|e| ws::Error::new(ws::ErrorKind::Custom(Box::new(e)), ""))?;
            if resp.rid.is_some() {
                let rid = resp.rid.as_ref().unwrap();
                log::info!("trd_ws got rid={} response: {}", rid, body);
                // let rid: i64 = rid.parse().unwrap_or_default();
                if let Some(cb) = self.callbacks.remove(rid) {
                    cb.unwrap()(self, &mut resp);
                }
            } else {
                match resp.subj.as_ref().map(|s| &s[..]) {
                    Some("onWallet") => {
                        let data: gmex_api::Wlt = serde_json::from_value(resp.data.unwrap_or_default())
                            .map_err(|e| ws::Error::new(ws::ErrorKind::Custom(Box::new(e)), ""))?;
                        log::info!("[trd.onWallet]: {:?}", data);
                    }
                    Some("onTrade") => {
                        let data: gmex_api::TrdRec = serde_json::from_value(resp.data.unwrap_or_default())
                            .map_err(|e| ws::Error::new(ws::ErrorKind::Custom(Box::new(e)), ""))?;
                        log::info!("[trd.onTrade]: {:?}", data);
                    }
                    Some("onOrder") => {
                        let data: gmex_api::Order = serde_json::from_value(resp.data.unwrap_or_default())
                            .map_err(|e| ws::Error::new(ws::ErrorKind::Custom(Box::new(e)), ""))?;
                        log::info!("[trd.onOrder]: {:?}", data);
                    }
                    Some("onPosition") => {
                        let data: gmex_api::Position = serde_json::from_value(resp.data.unwrap_or_default())
                            .map_err(|e| ws::Error::new(ws::ErrorKind::Custom(Box::new(e)), ""))?;
                        log::info!("[trd.onPosition]: {:?}", data);
                    }
                    Some("onWltLog") => {
                        let data: gmex_api::WltLog = serde_json::from_value(resp.data.unwrap_or_default())
                            .map_err(|e| ws::Error::new(ws::ErrorKind::Custom(Box::new(e)), ""))?;
                        log::info!("[trd.onWltLog]: {:?}", data);
                    }
                    _ => {
                        log::warn!("[-] unknow resp msg body: {}", body);
                    }
                }
            }
            Ok(())
        }
    }

    ws::connect(trade_base_url.as_str(), |out| {
        ws::deflate::DeflateBuilder::new()
            .with_settings(ws::deflate::DeflateSettings {
                max_window_bits: 15, // guide 中设置10，会导致大数据包时解压缩数据错误-3,默认的15暂时没问题.
                ..Default::default()
            })
            .build(Client::new(out, user_name, api_key, api_secret))
    })?;

    // Give the server a little time to get going
    sleep(Duration::from_millis(500));

    // TODO
    Ok(())
}

fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "info,websocket_client=debug"); // error, info, warn, or trace
    env_logger::init();

    let gmex_user_name = std::env::var("GMEX_USER_NAME").expect("GMEX_USER_NAME must be set");
    let gmex_api_key = std::env::var("GMEX_API_KEY").expect("GMEX_API_KEY must be set");
    let gmex_api_secret = std::env::var("GMEX_API_SECRET").expect("GMEX_API_SECRET must be set");
    //
    let gmex_ws_url_market: String =
        std::env::var("GMEX_WS_URL_MARKET").unwrap_or_else(|_| gmex_api::GMEX_WS_URL_MARKET.to_string());
    let gmex_ws_url_trade: String =
        std::env::var("GMEX_WS_URL_TRADE").unwrap_or_else(|_| gmex_api::GMEX_WS_URL_MARKET.to_string());

    log::debug!("GMEX-WebSocket-API Test....");
    log::debug!("  GMEX_WS_URL_MARKET = {}", gmex_ws_url_market);
    log::debug!("  GMEX_WS_URL_TRADE = {}", gmex_ws_url_trade);
    log::debug!("  GMEX_USER_NAME = {}", gmex_user_name);
    log::debug!("  GMEX_API_KEY = {}", gmex_api_key);
    log::debug!("  GMEX_API_SECRET = {}", gmex_api_secret);

    let mkt_cli = thread::Builder::new()
        .name("mkt_cli".to_owned())
        .spawn(move || websocket_market_demo(&gmex_ws_url_market))?;

    let trd_cli = thread::Builder::new()
        .name("trd_cli".to_owned())
        .spawn(move || websocket_trade_demo(&gmex_ws_url_trade, &gmex_user_name, &gmex_api_key, &gmex_api_secret))?;

    // Wait for the worker threads to finish what they are doing
    let _ = mkt_cli.join();
    let _ = trd_cli.join();

    log::debug!("[+] all done!");

    Ok(())
}
