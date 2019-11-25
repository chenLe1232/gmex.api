use log;
use reqwest;

use uuid::Uuid;

#[macro_use]
extern crate serde_json;

use serde::{Deserialize, Serialize};

#[macro_use]
extern crate failure;
use failure::Error;

use gmex_api;

fn http_market_demo(market_base_url: &String) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let base_url = market_base_url.trim_end_matches(|c| c == '/');
    if true {
        let rsp: serde_json::Value = client
            .get(&(base_url.to_owned() + "/ServerInfo"))
            .send()?
            .json()?;
        log::info!("行情服务器信息 = {:?}", rsp);
    }
    if true {
        // 获取服务器时间Time
        #[derive(Debug, Serialize, Deserialize)]
        struct Response {
            code: i32,
            time: i64,
            data: String,
        }
        let rsp: Response = client
            .get(&(base_url.to_owned() + "/Time?args=my-callback-data"))
            .send()?
            .json()?;
        log::info!("行情服务器时间 = {:?}", rsp);
    }
    if true {
        // 获取可订阅的指数信息 GetCompositeIndex
        let mut rsp: gmex_api::HttpResponseMessage<Vec<gmex_api::MktCompositeIndexTick>> = client
            .get(&(base_url.to_owned() + "/GetCompositeIndex"))
            .send()?
            .json()?;
        if rsp.has_error() {
            return Err(format_err!("GetCompositeIndex failed: {:?}", rsp));
        }
        let rows = rsp.take_rsp_data().unwrap_or_else(|| Default::default());
        log::info!("查询综合指数: {:?}", rows);
    }
    if true {
        // 获取可订阅的交易对信息 GetAssetD
        // 参数VP是子交易所的编号，默认为0是GAEA交易所.
        let mut rsp: gmex_api::HttpResponseMessage<Vec<gmex_api::AssetD>> = client
            .get(&(base_url.to_owned() + "/GetAssetD?VP=0"))
            .send()?
            .json()?;
        if rsp.has_error() {
            return Err(format_err!("GetAssetD failed: {:?}", rsp));
        }
        let mut rows = rsp.take_rsp_data().unwrap_or_else(|| Default::default());
        rows.sort_by(|a, b| {
            if a.TrdCls == b.TrdCls {
                a.Sym.cmp(&b.Sym)
            } else {
                // a.TrdCls.cmp(b.TrdCls)
                (a.TrdCls.unwrap_or_default() as i32).cmp(&(b.TrdCls.unwrap_or_default() as i32))
            }
        });

        log::info!("查询交易对信息返回 {} 个:", rows.len());
        for it in rows {
            log::info!("  {} {:?}", it.Sym.unwrap_or_default(), it.TrdCls.unwrap_or_default());
        }
    }

    if true {
        // 交易对信息扩展属性 GetAssetEx
        // 注意，如果交易对没有定义扩展属性，则这里返回的就没有这个交易对的数据.
        // 实际使用时，应该和 GetAssetD 配合一起使用，这里返回的结果和前面的进行结合.
        let mut rsp: gmex_api::HttpResponseMessage<Vec<gmex_api::V2AssetCfg>> = client
            .get(&(base_url.to_owned() + "/GetAssetEx?VP=0"))
            .send()?
            .json()?;
        if rsp.has_error() {
            return Err(format_err!("GetAssetEx failed: {:?}", rsp));
        }
        let rows = rsp.take_rsp_data().unwrap_or_else(|| Default::default());
        log::info!("查询交易对扩展属性返回 {} 个:", rows.len());
        for it in rows {
            // log::info!("  {:?}: {:?}", it.Sym.unwrap_or_default(), it);
            log::info!("  {}", it.Sym.unwrap_or_default());
        }
    }

    if true {
        // 获取历史K线数据 GetHistKLine
        let args = gmex_api::MktQueryKLineHistoryRequestArgs {
            Sym: Some(String::from("BTC.USDT")),
            Typ: Some(gmex_api::MktKLineType::KL_1m),
            Sec: Some(1541987816),
            Offset: Some(0),
            Count: Some(15),
        };
        let mut rsp: gmex_api::HttpResponseMessage<gmex_api::MktQueryKLineHistoryResult> = client
            .post(&(base_url.to_owned() + "/GetHistKLine"))
            .json(&args)
            .send()?
            .json()?;
        if rsp.has_error() {
            return Err(format_err!("GetHistKLine failed: {:?}", rsp));
        }
        let res = rsp.take_rsp_data().unwrap_or_else(|| Default::default());
        log::info!("获取历史K线数据: {:?}", res);
    }

    if true {
        // 获取最近K线数据 GetLatestKLine
        // 有些交易对不知道啥时候停牌了等原因导致没有k线数据，通过这个查询可以得到该交易对最近的一些数据.
        let mut rsp: gmex_api::HttpResponseMessage<gmex_api::MktQueryKLineHistoryResult> = client
            .post(&(base_url.to_owned() + "/GetLatestKLine"))
            .json(&json!({
                "Sym": "ETH.USDT",
                "Typ": "1h",
                "Count": 24,
            }))
            .send()?
            .json()?;
        if rsp.has_error() {
            return Err(format_err!("GetLatestKLine failed: {:?}", rsp));
        }
        let res = rsp.take_rsp_data().unwrap_or_else(|| Default::default());
        log::info!("获取最近K线数据: {:?}", res);
    }

    if true {
        // 指数的聚合行情 GetIndexTick
        let mut rsp: gmex_api::HttpResponseMessage<gmex_api::MktCompositeIndexTick> = client
            .post(&(base_url.to_owned() + "/GetIndexTick?idx=GMEX_CI_ETH"))
            .send()?
            .json()?;
        if rsp.has_error() {
            return Err(format_err!("GetIndexTick failed: {:?}", rsp));
        }
        let res = rsp.take_rsp_data().unwrap_or_else(|| Default::default());
        log::info!("指数的聚合行情 = {:?}", res);
    }

    if true {
        // 批量获取数据 GetIndexTickList, 一次获取多个，方便使用.
        let mut rsp: gmex_api::HttpResponseMessage<Vec<gmex_api::MktCompositeIndexTick>> = client
            .post(&(base_url.to_owned() + "/GetIndexTickList?idx_list=GMEX_CI_BTC,GMEX_CI_ETH"))
            .send()?
            .json()?;
        if rsp.has_error() {
            return Err(format_err!("GetIndexTickList failed: {:?}", rsp));
        }
        let res = rsp.take_rsp_data().unwrap_or_else(|| Default::default());
        log::info!("指数的聚合行情/批量 = {:?}", res);
    }

    if true {
        // 交易对的聚合行情 GetTick
        let mut rsp: gmex_api::HttpResponseMessage<gmex_api::MktInstrumentTick> = client
            .post(&(base_url.to_owned() + "/GetTick?sym=BTC.USDT"))
            .send()?
            .json()?;
        if rsp.has_error() {
            return Err(format_err!("GetTick failed: {:?}", rsp));
        }
        let res = rsp.take_rsp_data().unwrap_or_else(|| Default::default());
        log::info!("交易对的聚合行情 = {:?}", res);
    }
    if true {
        // 批量获取数据 GetTickList
        let mut rsp: gmex_api::HttpResponseMessage<Vec<gmex_api::MktInstrumentTick>> = client
            .post(
                &(base_url.to_owned() + "/GetTickList?sym_list=BTC.BTC,BTC.USDT,ETH.ETH,ETH.USDT"),
            )
            .send()?
            .json()?;
        if rsp.has_error() {
            return Err(format_err!("GetTickList failed: {:?}", rsp));
        }
        let res = rsp.take_rsp_data().unwrap_or_else(|| Default::default());
        log::info!("交易对的聚合行情/批量 = {:?}", res);
    }

    if true {
        // 获取交易对最近的成交记录 GetTrades
        let mut rsp: gmex_api::HttpResponseMessage<Vec<gmex_api::MktTradeItem>> = client
            .get(&(base_url.to_owned() + "/GetTrades?sym=BTC/USDT"))
            .send()?
            .json()?;
        if rsp.has_error() {
            return Err(format_err!("GetTrades failed: {:?}", rsp));
        }
        let res = rsp.take_rsp_data().unwrap_or_else(|| Default::default());
        log::info!("交易对最近的成交记录 = {:?}", res);
    }

    if true {
        // 获取交易对行情的20档盘口信息 GetOrd20
        let mut rsp: gmex_api::HttpResponseMessage<gmex_api::MktOrder20Result> = client
            .get(&(base_url.to_owned() + "/GetOrd20?sym=ETH/USDT"))
            .send()?
            .json()?;
        if rsp.has_error() {
            return Err(format_err!("GetOrd20 failed: {:?}", rsp));
        }
        let res = rsp.take_rsp_data().unwrap_or_else(|| Default::default());
        log::info!("交易对行情的20档盘口信息 = {:?}", res);
    }

    if true {
        // 批量获取数据 GetOrd20List, 一次获取多个方便使用.
        let mut rsp: gmex_api::HttpResponseMessage<Vec<gmex_api::MktOrder20Result>> = client
            .get(
                &(base_url.to_owned() + "/GetOrd20List?sym_list=BTC.BTC,BTC.USDT,ETH.ETH,ETH.USDT"),
            )
            .send()?
            .json()?;
        if rsp.has_error() {
            return Err(format_err!("GetOrd20List failed: {:?}", rsp));
        }
        let res = rsp.take_rsp_data().unwrap_or_else(|| Default::default());
        log::info!("交易对行情的20档盘口信息/批量 = {:?}", res);
    }

    Ok(())
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
        // 获取服务器信息
        let res: serde_json::Value = client
            .get(&(base_url.to_owned() + "/ServerInfo"))
            .send()?
            .json()?;
        log::info!("交易服务器信息 = {:?}", res);
    }

    if true {
        // 获取服务器时间Time
        #[derive(Debug, Serialize, Deserialize)]
        struct Response {
            code: i32,
            time: i64,
            data: Option<String>,
        }
        let rsp: Response = client
            .get(&(base_url.to_owned() + "/Time?args=my-callback-data"))
            .send()?
            .json()?;
        log::info!("交易服务器时间 = {:?}", rsp);
    }

    let mut userid: String = Default::default();
    if true {
        // 获取用户信息 GetUserInfo
        #[derive(Debug, Serialize, Deserialize)]
        struct Args {}

        #[derive(Debug, Default, Serialize, Deserialize)]
        struct UserInfo {
            #[serde(rename = "UserID")]
            userid: String,
            #[serde(rename = "UserName")]
            username: Option<String>,
        }

        let req = gmex_api::HttpTradeRequestMessage::new(
            "GetUserInfo",
            Args {},
            user_name,
            api_key,
            api_secret,
        );
        let mut rsp: gmex_api::HttpResponseMessage<UserInfo> = client
            .post(&(base_url.to_owned() + "/Action"))
            .json(&req)
            .send()?
            .json()?;
        if rsp.has_error() {
            return Err(format_err!("GetUserInfo failed: {:?}", rsp));
        }
        let myinfo = rsp.take_rsp_data().unwrap_or_else(|| Default::default());
        log::info!(
            "交易服务器获取用户信息成功, UserID: {}, UserName: {}",
            myinfo.userid,
            myinfo.username.unwrap_or_default(),
        );
        userid = myinfo.userid;
    }

    if true {
        // 查询资金中心(我的钱包)的钱包信息 GetCcsWallets
        #[derive(Debug, Serialize, Deserialize)]
        struct Args {}
        let req = gmex_api::HttpTradeRequestMessage::new(
            "GetCcsWallets",
            Args {},
            user_name,
            api_key,
            api_secret,
        );
        let mut rsp: gmex_api::HttpResponseMessage<Vec<gmex_api::CcsMainWallet>> = client
            .post(&(base_url.to_owned() + "/Action"))
            .json(&req)
            .send()?
            .json()?;
        if rsp.has_error() {
            return Err(format_err!("GetCcsWallets failed: {:?}", rsp));
        }
        let rows = rsp.take_rsp_data().unwrap_or_else(|| Default::default());
        log::info!("查询资金中心(我的钱包)的钱包信息: {:?}", rows);
    }

    if true {
        // 获取合约和币币的钱包信息 GetWallets
        #[derive(Debug, Serialize, Deserialize)]
        struct Args {
            #[serde(rename = "AId")]
            aid: String,
        }
        let req = gmex_api::HttpTradeRequestMessage::new(
            "GetWallets",
            Args {
                aid: format!("{}01", userid),
            },
            user_name,
            api_key,
            api_secret,
        );
        let mut rsp: gmex_api::HttpResponseMessage<Vec<gmex_api::Wlt>> = client
            .post(&(base_url.to_owned() + "/Action"))
            .json(&req)
            .send()?
            .json()?;
        if rsp.has_error() {
            return Err(format_err!("GetWallets-01 failed: {:?}", rsp));
        }
        let rows = rsp.take_rsp_data().unwrap_or_else(|| Default::default());
        log::info!("获取合约的钱包信息: {:?}", rows);

        let req = gmex_api::HttpTradeRequestMessage::new(
            "GetWallets",
            Args {
                aid: format!("{}02", userid),
            },
            user_name,
            api_key,
            api_secret,
        );
        let mut rsp: gmex_api::HttpResponseMessage<Vec<gmex_api::Wlt>> = client
            .post(&(base_url.to_owned() + "/Action"))
            .json(&req)
            .send()?
            .json()?;
        if rsp.has_error() {
            return Err(format_err!("GetWallets-02 failed: {:?}", rsp));
        }
        let rows = rsp.take_rsp_data().unwrap_or_else(|| Default::default());
        log::info!("获取币币的钱包信息: {:?}", rows);
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
        // 查询合约持仓信息 GetPositions
        #[derive(Debug, Serialize, Deserialize)]
        struct Args {
            #[serde(rename = "AId")]
            aid: String,
        }
        let req = gmex_api::HttpTradeRequestMessage::new(
            "GetPositions",
            Args {
                aid: format!("{}01", userid),
            },
            user_name,
            api_key,
            api_secret,
        );
        let mut rsp: gmex_api::HttpResponseMessage<Vec<gmex_api::Position>> = client
            .post(&(base_url.to_owned() + "/Action"))
            .json(&req)
            .send()?
            .json()?;
        if rsp.has_error() {
            return Err(format_err!("GetPositions failed: {:?}", rsp));
        }
        let rows = rsp.take_rsp_data().unwrap_or_else(|| Default::default());
        log::info!("查询合约账户当前持仓: {:?}", rows);
    }
    if true {
        // 查询合约委托 GetOrders
        #[derive(Debug, Serialize, Deserialize)]
        struct Args {
            #[serde(rename = "AId")]
            aid: String,
        }
        let req = gmex_api::HttpTradeRequestMessage::new(
            "GetOrders",
            Args {
                aid: format!("{}01", userid), // 要查询现货市场的委托，01改为02即可.
            },
            user_name,
            api_key,
            api_secret,
        );
        let mut rsp: gmex_api::HttpResponseMessage<Vec<gmex_api::Ord>> = client
            .post(&(base_url.to_owned() + "/Action"))
            .json(&req)
            .send()?
            .json()?;
        if rsp.has_error() {
            return Err(format_err!("GetOrders failed: {:?}", rsp));
        }
        let rows = rsp.take_rsp_data().unwrap_or_else(|| Default::default());
        log::info!("查询合约账户当前有效委托: {:?}", rows);
    }
    if true {
        // 合约委托下单 OrderNew
        let myord = gmex_api::Ord {
            AId: Some(format!("{}01", userid)),
            COrdId: Some(Uuid::new_v4().to_simple().to_string()),
            Sym: Some("BTC.USDT".to_string()),
            Dir: Some(gmex_api::OrderDir::BID),
            OType: Some(gmex_api::OfferType::Limit),
            Prz: Some(gmex_api::dec!(15_000)),
            Qty: Some(gmex_api::dec!(1)),
            ..Default::default()
        };

        if false {
            // 看看序列化结果...
            let s1 = serde_json::to_string(&myord).unwrap();
            log::debug!("[OrderNew->JSON]: {}", s1);
        }

        let req = gmex_api::HttpTradeRequestMessage::new(
            "OrderNew", myord, user_name, api_key, api_secret,
        );
        let mut rsp: gmex_api::HttpResponseMessage<gmex_api::Ord> = client
            .post(&(base_url.to_owned() + "/Action"))
            .json(&req)
            .send()?
            .json()?;
        if rsp.has_error() {
            return Err(format_err!("OrderNew failed: {:?}", rsp));
        }
        let rows = rsp.take_rsp_data().unwrap_or_else(|| Default::default());
        log::info!("合约账户下单 OrderNew 返回: {:?}", rows);
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
        .unwrap_or_else(|_| gmex_api::GMEX_HTTP_URL_MARKET.to_string());
    let gmex_http_url_trade: String = std::env::var("GMEX_HTTP_URL_TRADE")
        .unwrap_or_else(|_| gmex_api::GMEX_WS_URL_TRADE.to_string());
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
