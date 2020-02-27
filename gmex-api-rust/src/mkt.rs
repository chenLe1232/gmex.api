// GMEX-API 行情相关结构体定义
//

use serde::{Deserialize, Serialize};

use crate::helper::is_default;
use crate::types::OrderDir;

/** [综合指数] 的Tick行情 */
#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MktCompositeIndexTick {
    #[serde(skip_serializing_if = "is_default")] pub Sym: String,      // 符号, 如 GMEX_CI_BTC, GMEX_CI_ETH, GMEX_CI_GAEA ...
    #[serde(skip_serializing_if = "is_default")] pub At: i64,          // 时间(ms)
    #[serde(skip_serializing_if = "is_default")] pub Prz: f64,         // 最新价
    #[serde(skip_serializing_if = "is_default")] pub Sz: f64,          // 成交量
    //
    #[serde(skip_serializing_if = "is_default")] pub Prz24: f64,       // 24小时初始价格
    #[serde(skip_serializing_if = "is_default")] pub High24: f64,      // 24小时最高价
    #[serde(skip_serializing_if = "is_default")] pub Low24: f64,       // 24小时最低价
    #[serde(skip_serializing_if = "is_default")] pub Volume24: f64,    // 24小时成交量
    #[serde(skip_serializing_if = "is_default")] pub Turnover24: f64,  // 24小时总成交额
    //
    // pub RefThirdParty: serde_json::Value,  // 第三方参考数据
}

/** [交易对/合约] 的tick行情消息 */
#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MktInstrumentTick {
    #[serde(skip_serializing_if = "is_default")] pub Sym: String,      // 交易对符号
    #[serde(skip_serializing_if = "is_default")] pub At: i64,          // 时间戳,单位:毫秒
    // 
    #[serde(skip_serializing_if = "is_default")] pub PrzBid1: f64,     // 买1价
    #[serde(skip_serializing_if = "is_default")] pub SzBid1: f64,      // 买1量
    #[serde(skip_serializing_if = "is_default")] pub SzBid: f64,       // 总买量
    //
    #[serde(skip_serializing_if = "is_default")] pub PrzAsk1: f64,     // 卖1价
    #[serde(skip_serializing_if = "is_default")] pub SzAsk1: f64,      // 卖1量
    #[serde(skip_serializing_if = "is_default")] pub SzAsk: f64,       // 总卖量
    //
    #[serde(skip_serializing_if = "is_default")] pub LastPrz: f64,     // 最新成交价
    #[serde(skip_serializing_if = "is_default")] pub SettPrz: f64,     // 最新标记价格
    //
    #[serde(skip_serializing_if = "is_default")] pub Prz24: f64,       // 24小时初始价格
    #[serde(skip_serializing_if = "is_default")] pub High24: f64,      // 24小时最高价
    #[serde(skip_serializing_if = "is_default")] pub Low24: f64,       // 24小时最低价
    #[serde(skip_serializing_if = "is_default")] pub Volume24: f64,    // 24小时成交量
    #[serde(skip_serializing_if = "is_default")] pub Turnover24: f64,  // 24小时总成交额
    //
    #[serde(skip_serializing_if = "is_default")] pub Volume: f64,      // 总成交量
    #[serde(skip_serializing_if = "is_default")] pub Turnover: f64,    // 总成交额
    #[serde(skip_serializing_if = "is_default")] pub OpenInterest: f64,    // 总持仓量
    //
    #[serde(skip_serializing_if = "is_default")] pub FundingLongR: f64,    // 多仓资金费率
    #[serde(skip_serializing_if = "is_default")] pub FundingShortR: f64,   // 空仓资金费率 -- 暂时没用
    #[serde(skip_serializing_if = "is_default")] pub FundingPredictedR: f64,   // 预测费率
}

/** 行情里，订阅全深度后，推送过来的一行的消息 */
#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MktOrderItem {
    #[serde(skip_serializing_if = "is_default")] pub Sym: String,      // 交易对符号
    #[serde(skip_serializing_if = "is_default")] pub At: i64,          // 时间戳,单位:毫秒
    #[serde(skip_serializing_if = "is_default")] pub Prz: f64,         // 价格
    #[serde(skip_serializing_if = "is_default")] pub Dir: OrderDir,    // 方向; 1:BID, -1:ASK
    #[serde(skip_serializing_if = "is_default")] pub Sz: i32,          // 量
}

/** 行情里，订阅成交后，推送过来的消息 */
#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MktTradeItem {
    #[serde(skip_serializing_if = "is_default")] pub Sym: String,        // 交易对符号
    #[serde(skip_serializing_if = "is_default")] pub At: i64,            // 时间戳,单位:毫秒
    #[serde(skip_serializing_if = "is_default")] pub Prz: f64,           // 价格
    #[serde(skip_serializing_if = "is_default")] pub Dir: OrderDir,      // 方向; 1:BID, -1:ASK
    #[serde(skip_serializing_if = "is_default")] pub Sz: f64,            // 量
    #[serde(skip_serializing_if = "is_default")] pub Val: f64,           // 价值
    #[serde(skip_serializing_if = "is_default")] pub MatchID: String,    // 撮合ID
}

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
    #[serde(rename = "")]
    KL_Nil,
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
        self::MktKLineType::KL_Nil
    }
}

/** 行情里，一个 KLine 的数据结构 */
#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MktKLineItem {
    #[serde(skip_serializing_if = "is_default")] pub Sym: String,      // 交易对符号
    #[serde(skip_serializing_if = "is_default")] pub Typ: MktKLineType,// K线类型
    #[serde(skip_serializing_if = "is_default")] pub Sec: i64,         // 时间戳,单位:秒
    #[serde(skip_serializing_if = "is_default")] pub At: i64,          // 时间戳,单位:毫秒,最后更新时间，也是最后该kline时间段内最后一个成交记录的时间.
    #[serde(skip_serializing_if = "is_default")] pub PrzOpen: f64,     // 开始价
    #[serde(skip_serializing_if = "is_default")] pub PrzClose: f64,    // 结束价
    #[serde(skip_serializing_if = "is_default")] pub PrzHigh: f64,     // 最高价
    #[serde(skip_serializing_if = "is_default")] pub PrzLow: f64,      // 最低价
    #[serde(skip_serializing_if = "is_default")] pub Volume: f64,      // 总成交量
    #[serde(skip_serializing_if = "is_default")] pub Turnover: f64,    // 总成交额
}

/** 行情里，订阅20档深度后，推送过来的消息 */
#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MktOrder20Result {
    #[serde(skip_serializing_if = "is_default")] pub Sym: String,          // 交易对符号
    #[serde(skip_serializing_if = "is_default")] pub At: i64,              // 时间戳,单位:毫秒
    #[serde(skip_serializing_if = "Option::is_none")] pub Asks: Option<Vec<[f64; 2]>>,  // 卖盘口
    #[serde(skip_serializing_if = "Option::is_none")] pub Bids: Option<Vec<[f64; 2]>>,  // 买盘口
}

/** 行情里, 查询 KLine 历史数据时的请求参数 */
#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MktQueryKLineHistoryRequestArgs {
    #[serde(skip_serializing_if = "is_default")] pub Sym: String,          // 交易对符号
    #[serde(skip_serializing_if = "is_default")] pub Typ: MktKLineType,    // K线类型
    #[serde(skip_serializing_if = "is_default")] pub Sec: i64,             // 时间戳,单位:秒
    #[serde(skip_serializing_if = "is_default")] pub Offset: i64,          // 偏移量
    #[serde(skip_serializing_if = "is_default")] pub Count: i64,           // 数量, 最大 3000
}

/** 行情里, 查询KLine返回的结果 */
#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MktQueryKLineHistoryResult {
    #[serde(skip_serializing_if = "is_default")] pub Sym: String,          // 交易对符号
    #[serde(skip_serializing_if = "is_default")] pub Typ: MktKLineType,    // K线类型
    #[serde(skip_serializing_if = "is_default")] pub Count: i64,           // 返回结果的数量个数
    #[serde(skip_serializing_if = "is_default")] pub InitSec: i64,         // 这个交易对创建时间，最早的分钟kline的时间，早于这个时间肯定没有数据了.
    #[serde(skip_serializing_if = "Option::is_none")] pub Sec: Option<Vec<i64>>,        // 时间戳,单位:秒,数组
    #[serde(skip_serializing_if = "Option::is_none")] pub PrzOpen: Option<Vec<f64>>,    // 开始价
    #[serde(skip_serializing_if = "Option::is_none")] pub PrzClose: Option<Vec<f64>>,   // 结束价
    #[serde(skip_serializing_if = "Option::is_none")] pub PrzHigh: Option<Vec<f64>>,    // 最高价
    #[serde(skip_serializing_if = "Option::is_none")] pub PrzLow: Option<Vec<f64>>,     // 最低价
    #[serde(skip_serializing_if = "Option::is_none")] pub Volume: Option<Vec<f64>>,     // 总成交量
    #[serde(skip_serializing_if = "Option::is_none")] pub Turnover: Option<Vec<f64>>,   // 总成交额
}

/* ------------------------------------------------------------ */
/* ------------------------------------------------------------ */
/* ------------------------------------------------------------ */
