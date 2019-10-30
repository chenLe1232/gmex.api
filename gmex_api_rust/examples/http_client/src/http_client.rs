use log;
use md5;
use reqwest;

#[macro_use]
extern crate serde_json;

use serde::{Deserialize, Serialize};

#[macro_use]
extern crate failure;
use failure::Error;

use gmex_api::types as gmex_types;

fn http_market_demo(market_base_url: &String) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let base_url = market_base_url.trim_end_matches(|c| c == '/');
    if true {
        let res: serde_json::Value = client
            .get(&(base_url.to_owned() + "/ServerInfo"))
            .send()?
            .json()?;
        log::info!("行情服务器信息 = {:#?}", res);
    }
    if true {
        // 获取服务器时间Time
        #[derive(Debug, Serialize, Deserialize)]
        struct Response {
            code: i32,
            time: i64,
            data: String,
        }
        let res: Response = client
            .get(&(base_url.to_owned() + "/Time?args=my-callback-data"))
            .send()?
            .json()?;
        log::info!("行情服务器时间 = {:#?}", res);
    }
    if true {
        // 获取可订阅的指数信息 GetCompositeIndex
        #[derive(Debug, Serialize, Deserialize)]
        struct Response {
            code: i32,
            data: Vec<gmex_types::MktCompositeIndexTick>,
        }
        let res: Response = client
            .get(&(base_url.to_owned() + "/GetCompositeIndex"))
            .send()?
            .json()?;
        log::info!("综合指数 = {:#?}", res);
    }
    if true {
        // 获取可订阅的交易对信息 GetAssetD
        // 参数VP是子交易所的编号，默认为0是GAEA交易所.
        #[derive(Debug, Serialize, Deserialize)]
        struct Response {
            code: i32,
            data: Vec<gmex_types::AssetD>,
        }
        let mut res: Response = client
            .get(&(base_url.to_owned() + "/GetAssetD?VP=0"))
            .send()?
            .json()?;
        // log::info!("交易对 = {:?}", res);
        log::info!(
            "查询交易对信息返回: code={}, 交易对有 {} 个: ",
            res.code,
            res.data.len()
        );
        if res.code == 0 {
            res.data.sort_by(|a, b| {
                if a.TrdCls == b.TrdCls {
                    a.Sym.cmp(&b.Sym)
                } else {
                    (a.TrdCls as i32).cmp(&(b.TrdCls as i32))
                }
            });
            for it in res.data {
                log::info!("  {} {:?}", it.Sym, it.TrdCls);
            }
        }
    }

    if false {
        // 交易对信息扩展属性 GetAssetEx
        // 注意，很多交易对是没有扩展属性的，这里就没有了.
        // *NOTE*: 暂时没有，websocket 才有, FIXME!!! TODO!!!
        #[derive(Debug, Serialize, Deserialize)]
        struct Response {
            code: i32,
            data: Vec<gmex_types::V2AssetCfg>,
        }
        let res: Response = client
            .get(&(base_url.to_owned() + "/GetAssetEx?VP=0"))
            .send()?
            .json()?;
        log::info!("交易对扩展属性 = {:?}", res);
    }

    if true {
        // 获取历史K线数据 GetHistKLine
        let args = gmex_types::MktQueryKLineHistoryRequestArgs {
            Sym: "BTC.USDT".to_string(),
            Typ: "1m".to_string(),
            Sec: 1541987816,
            Offset: 0,
            Count: 15,
        };
        #[derive(Debug, Serialize, Deserialize)]
        struct Response {
            code: i32,
            data: gmex_types::MktQueryKLineHistoryResult,
        }
        let res: Response = client
            .post(&(base_url.to_owned() + "/GetHistKLine"))
            .json(&args)
            .send()?
            .json()?;
        log::info!("历史K线 = {:#?}", res);
    }

    if true {
        // 获取最近K线数据 GetLatestKLine
        #[derive(Debug, Serialize, Deserialize)]
        struct Response {
            code: i32,
            data: gmex_types::MktQueryKLineHistoryResult,
        }
        let res: Response = client
            .post(&(base_url.to_owned() + "/GetLatestKLine"))
            .json(&json!({
                "Sym": "ETH.USDT",
                "Typ": "1h",
                "Count": 24,
            }))
            .send()?
            .json()?;
        log::info!("最近K线 = {:#?}", res);
    }

    if true {
        // 指数的聚合行情 GetIndexTick
        #[derive(Debug, Serialize, Deserialize)]
        struct Response {
            code: i32,
            data: gmex_types::MktCompositeIndexTick,
        }
        let res: Response = client
            .post(&(base_url.to_owned() + "/GetIndexTick?idx=GMEX_CI_ETH"))
            .send()?
            .json()?;
        log::info!("指数的聚合行情 = {:#?}", res);
    }

    if true {
        // 批量获取数据 GetIndexTickList, 一次获取多个，方便使用.
        #[derive(Debug, Serialize, Deserialize)]
        struct Response {
            code: i32,
            data: Vec<gmex_types::MktCompositeIndexTick>,
        }
        let res: Response = client
            .post(&(base_url.to_owned() + "/GetIndexTickList?idx_list=GMEX_CI_BTC,GMEX_CI_ETH"))
            .send()?
            .json()?;
        log::info!("指数的聚合行情/批量 = {:#?}", res);
    }

    if true {
        // 交易对的聚合行情 GetTick
        #[derive(Debug, Serialize, Deserialize)]
        struct Response {
            code: i32,
            data: gmex_types::MktInstrumentTick,
        }
        let res: Response = client
            .post(&(base_url.to_owned() + "/GetTick?sym=BTC.USDT"))
            .send()?
            .json()?;
        log::info!("交易对的聚合行情 = {:#?}", res);
    }
    if true {
        // 批量获取数据 GetTickList
        #[derive(Debug, Serialize, Deserialize)]
        struct Response {
            code: i32,
            data: Vec<gmex_types::MktCompositeIndexTick>,
        }
        let res: Response = client
            .post(
                &(base_url.to_owned() + "/GetTickList?sym_list=BTC.BTC,BTC.USDT,ETH.ETH,ETH.USDT"),
            )
            .send()?
            .json()?;
        log::info!("交易对的聚合行情/批量 = {:#?}", res);
    }

    if true {
        // 获取交易对最近的成交记录 GetTrades
        #[derive(Debug, Serialize, Deserialize)]
        struct Response {
            code: i32,
            data: Vec<gmex_types::MktTradeItem>,
        }
        let res: Response = client
            .get(&(base_url.to_owned() + "/GetTrades?sym=BTC/USDT"))
            .send()?
            .json()?;
        log::info!("交易对最近的成交记录 = {:#?}", res);
    }

    if true {
        // 获取交易对行情的20档盘口信息 GetOrd20
        #[derive(Debug, Serialize, Deserialize)]
        struct Response {
            code: i32,
            data: gmex_types::MktOrder20Result,
        }
        let res: Response = client
            .get(&(base_url.to_owned() + "/GetOrd20?sym=ETH/USDT"))
            .send()?
            .json()?;
        log::info!("交易对行情的20档盘口信息 = {:#?}", res);
    }

    if true {
        // 批量获取数据 GetOrd20List, 一次获取多个方便使用.
        #[derive(Debug, Serialize, Deserialize)]
        struct Response {
            code: i32,
            data: Vec<gmex_types::MktOrder20Result>,
        }
        let res: Response = client
            .get(
                &(base_url.to_owned() + "/GetOrd20List?sym_list=BTC.BTC,BTC.USDT,ETH.ETH,ETH.USDT"),
            )
            .send()?
            .json()?;
        log::info!("交易对行情的20档盘口信息/批量 = {:#?}", res);
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct RestTradeMessageRequest<T> {
    req: String,
    args: T,
    expires: i64,
    username: String,
    apikey: String,
    signature: String,
}

use std::time::{SystemTime, UNIX_EPOCH};

impl<T> RestTradeMessageRequest<T>
where
    T: Serialize,
{
    pub fn new(
        req: &str,
        args: T,
        uname: &String,
        api_key: &String,
        api_secret: &String,
    ) -> Self {
        let now_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        let expires = (now_ms as i64) + 5000; // 设置5秒过期, FIXME!!!
        let s1 = serde_json::to_string(&args).unwrap();
        // 签名计算公式: MD5(req+args+expires+API.SecretKey)
        let txt = format!("{}{}{}{}", req, s1, expires, api_secret);
        let digest = md5::compute(txt);
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

fn http_trade_demo(
    trade_base_url: &String,
    user_name: &String,
    api_key: &String,
    api_secret: &String,
) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let base_url = trade_base_url.trim_end_matches(|c| c == '/');
    if true {
        let res: serde_json::Value = client
            .get(&(base_url.to_owned() + "/ServerInfo"))
            .send()?
            .json()?;
        log::info!("交易服务器信息 = {:#?}", res);
    }

    if true {
        // 获取服务器时间Time
        #[derive(Debug, Serialize, Deserialize)]
        struct Response {
            code: i32,
            time: i64,
            data: String,
        }
        let res: Response = client
            .get(&(base_url.to_owned() + "/Time?args=my-callback-data"))
            .send()?
            .json()?;
        log::info!("交易服务器时间 = {:#?}", res);
    }

    let mut userid: String = String::new();
    if true {
        // 获取用户信息 GetUserInfo
        #[derive(Debug, Serialize, Deserialize)]
        struct Args {}

        #[derive(Debug, Serialize, Deserialize)]
        struct UserInfo {
            #[serde(rename = "UserID")]
            userid: String,
            #[serde(rename = "UserName")]
            username: String,
        }
        #[derive(Debug, Serialize, Deserialize)]
        struct Response {
            code: i32,
            data: UserInfo,
        }
        let req =
            RestTradeMessageRequest::new("GetUserInfo", Args {}, user_name, api_key, api_secret);

        let res: Response = client
            .post(&(base_url.to_owned() + "/Action"))
            .json(&req)
            .send()?
            .json()?;
        if res.code == 0 {
            log::info!(
                "交易服务器获取用户信息成功,UserID: {}, UserName: {}",
                res.data.userid,
                res.data.username
            );
            userid = res.data.userid;
        } else {
            log::warn!("交易服务器获取用户信息失败: {:#?}", res.code);
        }
    }

    if true {
        // 查询资金中心(我的钱包)的钱包信息 GetCcsWallets
        #[derive(Debug, Serialize, Deserialize)]
        struct Args {}
        let req =
            RestTradeMessageRequest::new("GetCcsWallets", Args {}, user_name, api_key, api_secret);

        let res: serde_json::Value = client
            .post(&(base_url.to_owned() + "/Action"))
            .json(&req)
            .send()?
            .json()?;
        log::info!("查询资金中心(我的钱包)的钱包信息: {:#?}", res);
    }

    if true {
        // 获取合约和币币的钱包信息 GetWallets
        #[derive(Debug, Serialize, Deserialize)]
        struct Args {
            #[serde(rename = "AId")]
            aid: String,
        }
        let req = RestTradeMessageRequest::new(
            "GetWallets",
            Args {
                aid: format!("{}01", userid),
            },
            user_name,
            api_key,
            api_secret,
        );

        let res: serde_json::Value = client
            .post(&(base_url.to_owned() + "/Action"))
            .json(&req)
            .send()?
            .json()?;
        log::info!("获取合约的钱包信息: {:#?}", res);

        let req = RestTradeMessageRequest::new(
            "GetWallets",
            Args {
                aid: format!("{}02", userid),
            },
            user_name,
            api_key,
            api_secret,
        );
        let res: serde_json::Value = client
            .post(&(base_url.to_owned() + "/Action"))
            .json(&req)
            .send()?
            .json()?;
        log::info!("获取币币的钱包信息: {:#?}", res);
    }
    if true {
        // 查询最近的钱包日志 GetWalletsLog, args: {AId: 账号AId,合约钱包AId=UserID+'01',现货钱包AId=UserID+'02' }
        // TODO
    }
    if true {
        // 查询最近的历史订单明细 GetHistOrders
        // TODO
    }
    if true {
        // 根据OrdID来查询委托单 GetOrderByID
        // TODO
    }
    if true {
        // 查询持仓信息 GetPositions
        // TODO
    }
    if true {
        // 查询委托 GetOrders
        // TODO
    }
    if true {
        // 委托下单 OrderNew
        // TODO
    }
    if true {
        // 撤销委托 OrderDel
        // TODO
    }
    if true {
        // 调整杠杆 PosLeverage
        // TODO
    }
    if true {
        // 增删资金 PosTransMgn
        // TODO
    }
    if true {
        // 超时撤单 CancelAllAfter
        // TODO
    }
    if true {
        // 获取用户风险限额 GetRiskLimit
        // TODO
    }
    if true {
        // 获取用户风险限额 GetRiskLimit
        // TODO
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "info,http_client=debug"); // error, info, warn, or trace
    env_logger::init();

    let gmex_user_name = std::env::var("GMEX_USER_NAME").expect("GMEX_USER_NAME must be set");
    let gmex_api_key = std::env::var("GMEX_API_KEY").expect("GMEX_API_KEY must be set");
    let gmex_api_secret = std::env::var("GMEX_API_SECRET").expect("GMEX_API_SECRET must be set");
    //
    let gmex_http_url_market: String = std::env::var("GMEX_HTTP_URL_MARKET")
        .unwrap_or_else(|_| "https://api-market.gmex.io/v1/rest".to_string());
    let gmex_http_url_trade: String = std::env::var("GMEX_HTTP_URL_TRADE")
        .unwrap_or_else(|_| "https://api-trade.gmex.io/v1/rest".to_string());

    log::debug!("GMEX-HTTP-API Test....");
    log::debug!("  GMEX_HTTP_URL_MARKET = {}", gmex_http_url_market);
    log::debug!("  GMEX_HTTP_URL_TRADE = {}", gmex_http_url_trade);
    log::debug!("  GMEX_USER_NAME = {}", gmex_user_name);
    log::debug!("  GMEX_API_KEY = {}", gmex_api_key);
    log::debug!("  GMEX_API_SECRET = {}", gmex_api_secret);

    if true {
        http_market_demo(&gmex_http_url_market)?;
    }
    if true {
        http_trade_demo(
            &gmex_http_url_trade,
            &gmex_user_name,
            &gmex_api_key,
            &gmex_api_secret,
        )?;
    }

    Ok(())
}
