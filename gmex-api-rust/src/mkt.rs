// GMEX-API 行情相关结构体定义
//

use serde::{Deserialize, Serialize};
use super::types::OrderDir;

/** [综合指数] 的Tick行情 */
#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MktCompositeIndexTick {
    pub Sym: Option<String>,        // 符号, 如 GMEX_CI_BTC, GMEX_CI_ETH, GMEX_CI_GAEA ...
    pub At: Option<i64>,            // 时间(ms)
    pub Prz: Option<f64>,           // 最新价
    pub Sz: Option<f64>,            // 成交量
    //
    pub Prz24: Option<f64>,         // 24小时初始价格
    pub High24: Option<f64>,        // 24小时最高价
    pub Low24: Option<f64>,         // 24小时最低价
    pub Volume24: Option<f64>,      // 24小时成交量
    pub Turnover24: Option<f64>,    // 24小时总成交额
    //
    // pub RefThirdParty: serde_json::Value,  // 第三方参考数据
}

/** [交易对/合约] 的tick行情消息 */
#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MktInstrumentTick {
    pub Sym: Option<String>,    // 交易对符号
    pub At: Option<i64>,        // 时间戳,单位:毫秒
    // 
    pub PrzBid1: Option<f64>,   // 买1价
    pub SzBid1: Option<f64>,    // 买1量
    pub SzBid: Option<f64>,     // 总买量
    //
    pub PrzAsk1: Option<f64>,   // 卖1价
    pub SzAsk1: Option<f64>,    // 卖1量
    pub SzAsk: Option<f64>,     // 总卖量
    //
    pub LastPrz: Option<f64>,   // 最新成交价
    pub SettPrz: Option<f64>,   // 最新标记价格
    //
    pub Prz24: Option<f64>,         // 24小时初始价格
    pub High24: Option<f64>,        // 24小时最高价
    pub Low24: Option<f64>,         // 24小时最低价
    pub Volume24: Option<f64>,      // 24小时成交量
    pub Turnover24: Option<f64>,    // 24小时总成交额
    //
    pub Volume: Option<f64>,        // 总成交量
    pub Turnover: Option<f64>,      // 总成交额
    pub OpenInterest: Option<f64>,  // 总持仓量
    //
    pub FundingLongR: Option<f64>,      // 多仓资金费率
    pub FundingShortR: Option<f64>,     // 空仓资金费率 -- 暂时没用
    pub FundingPredictedR: Option<f64>, // 预测费率
}

/** 行情里，订阅全深度后，推送过来的一行的消息 */
#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MktOrderItem {
    pub Sym: Option<String>,    // 交易对符号
    pub At: Option<i64>,        // 时间戳,单位:毫秒
    pub Prz: Option<f64>,       // 价格
    pub Dir: Option<i32>,       // 方向; 1:BID, -1:ASK
    pub Sz: Option<i32>,        // 量
}

/** 行情里，订阅成交后，推送过来的消息 */
#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MktTradeItem {
    pub Sym: Option<String>,        // 交易对符号
    pub At: Option<i64>,            // 时间戳,单位:毫秒
    pub Prz: Option<f64>,           // 价格
    pub Dir: Option<OrderDir>,      // 方向; 1:BID, -1:ASK
    pub Sz: Option<f64>,            // 量
    pub Val: Option<f64>,           // 价值
    pub MatchID: Option<String>,    // 撮合ID
}

/*
KLine/K线/K柱 的类型
支持的类型有: 1m, 3m, 5m, 15m, 30m, 1h, 2h, 4h, 6h, 8h, 12h, 1d, 3d, 1w, 2w, 1M
Kline/Candlestick chart intervals: m -> minutes; h -> hours; d -> days; w -> weeks; M -> months
pub enum MktKLineType
TODO

*/

/** 行情里，KLine/K线/K柱 的类型 */
/*
支持的类型有: 1m, 3m, 5m, 15m, 30m, 1h, 2h, 4h, 6h, 8h, 12h, 1d, 3d, 1w, 2w, 1M
Kline/Candlestick chart intervals: m -> minutes; h -> hours; d -> days; w -> weeks; M -> months
NOTE: 服务器端应该只提供 1m,5m,1h,1d 四种即可，其它的客户端自己组合.
*/
#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum MktKLineType {
    #[serde(rename = "1m")]
    KL_1m,
    #[serde(rename = "3m")]
    KL_3m,
    #[serde(rename = "5m")]
    KL_5m,
    #[serde(rename = "15m")]
    KL_15m,
    #[serde(rename = "30m")]
    KL_30m,
    #[serde(rename = "1h")]
    KL_1h,
    #[serde(rename = "2h")]
    KL_2h,
    #[serde(rename = "4h")]
    KL_4h,
    #[serde(rename = "6h")]
    KL_6h,
    #[serde(rename = "8h")]
    KL_8h,
    #[serde(rename = "12h")]
    KL_12h,
    #[serde(rename = "1d")]
    KL_1d,
    #[serde(rename = "3d")]
    KL_3d,
    #[serde(rename = "1w")]
    KL_1w,
    #[serde(rename = "2w")]
    KL_2w,
    #[serde(rename = "1M")]
    KL_1M,
}

impl Default for MktKLineType {
    fn default() -> Self {
        self::MktKLineType::KL_1m
    }
}

/** 行情里，一个 KLine 的数据结构 */
#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MktKLineItem {
    pub Sym: Option<String>,            // 交易对符号
    pub Typ: Option<MktKLineType>,      // K线类型
    pub Sec: Option<i64>,       // 时间戳,单位:秒
    pub PrzOpen: Option<f64>,   // 开始价
    pub PrzClose: Option<f64>,  // 结束价
    pub PrzHigh: Option<f64>,   // 最高价
    pub PrzLow: Option<f64>,    // 最低价
    pub Volume: Option<f64>,    // 总成交量
    pub Turnover: Option<f64>,  // 总成交额
}

/** 行情里，订阅20档深度后，推送过来的消息 */
#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MktOrder20Result {
    pub Sym: Option<String>,            // 交易对符号
    pub At: Option<i64>,                // 时间戳,单位:毫秒
    pub Asks: Option<Vec<[f64; 2]>>,    // 卖
    pub Bids: Option<Vec<[f64; 2]>>,    // 买
}

/** 行情里, 查询 KLine 历史数据时的请求参数 */
#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MktQueryKLineHistoryRequestArgs {
    pub Sym: Option<String>,    // 交易对符号
    pub Typ: Option<MktKLineType>,    // K线类型
    pub Sec: Option<i64>,       // 时间戳,单位:秒
    pub Offset: Option<i64>,    // 偏移量
    pub Count: Option<i64>,     // 数量, 最大 3000
}

/** 行情里, 查询KLine返回的结果 */
#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MktQueryKLineHistoryResult {
    pub Sym: Option<String>,        // 交易对符号
    pub Typ: Option<MktKLineType>,  // K线类型
    pub Count: Option<i64>,         // 返回结果的数量个数
    pub Sec: Option<Vec<i64>>,      // 时间戳,单位:秒,数组
    pub PrzOpen: Option<Vec<f64>>,  // 开始价
    pub PrzClose: Option<Vec<f64>>, // 结束价
    pub PrzHigh: Option<Vec<f64>>,  // 最高价
    pub PrzLow: Option<Vec<f64>>,   // 最低价
    pub Volume: Option<Vec<f64>>,   // 总成交量
    pub Turnover: Option<Vec<f64>>, // 总成交额
}
