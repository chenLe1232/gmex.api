// GMEX-API 数据结构定义

use super::Decimal;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

// #[macro_use]
// extern crate bitflags;

// fn is_default<T: Default + PartialEq>(t: &T) -> bool {
//     t == &T::default()
// }

/* 委托单方向 买/卖 */
// #[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum OrderDir {
    Nil = 0,
    BID = 1,
    // BUY = 1,
    ASK = -1,
    // SLEE = -1,
}

impl Default for OrderDir {
    fn default() -> OrderDir {
        // OrderDir::from_i32(0).unwrap()
        OrderDir::Nil
    }
}

/* 来源: 用于 委托、交易、结算等 */
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum OrderVia {
    Nil = 0,
    Web = 1,
    App = 2,
    Api = 3,
    Liquidate = 4,  // 平仓 Liquidate
    ADLEngine = 5,  // ADL 减仓操作
    Settlement = 6, // 结算
    Trade = 7,      // 交易
    Fee = 8,        // 手续费
    Depo = 9,       // 存
    Wdrw = 10,      // 取
    Funding = 11,   // Funding 资金费率
    Offer = 12,     // 配售
    Gift_Give = 17, // 给予Gift
    Wlt_Settle = 18,    // 钱包结算
    Gift_Settle = 19,   // 礼金结算
}

impl Default for OrderVia {
    fn default() -> OrderVia {
        OrderVia::Nil
    }
}

/* 委托的状态 */
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum OrderStatus {
    Nil = 0,
    Queueing = 1, // 正在排队
    Matching = 2, // 有效
    PostFail = 3, // 提交失败
    Executed = 4, // 已执行
}

impl Default for OrderStatus {
    fn default() -> OrderStatus {
        OrderStatus::Nil
    }
}

/* 报价方式 */
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum OfferType {
    Nil = 0,
    Limit = 1,       // 限价委单
    Market = 2,      // 市价委单,匹配后转限价
    StopLimit = 3,   // 限价止损/盈利
    StopMarket = 4,  // 市价止损/盈利
    TraceLimit = 5,  // 追踪 限价
    TraceMarket = 6, // 追踪 市价
}

impl Default for OfferType {
    fn default() -> OfferType {
        OfferType::Nil
    }
}

/* 条件委托触发的判据 */
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum StopBy {
    PriceMark = 0,   // 标记价格
    PriceLatest = 1, // 最新成交
    PriceIndex = 2,  // 指数价格
}

impl Default for StopBy {
    fn default() -> StopBy {
        StopBy::PriceMark
    }
}

/* 交易指令的标志 */
bitflags! {
    #[derive(Default, Serialize, Deserialize)]
    pub struct OrdFlags: u32 {
        const POSTONLY          = 0x00000001; // 如果委托会立即成交，则不发送此委托
        const REDUCEONLY        = 0x00000002; // 如果委托会导致增加仓位，则不发送此委托
        const CLOSEONTRIGGER    = 0x00000004; // 触发后平仓 TODO 目前未实现
        const IF_GREATERTHAN    = 0x00000008; // 条件指定为 如果价格大于StopBy
        const IF_LESSTHAN       = 0x00000010; // 条件指定为 如果价格低于StopBy
        const TRACE_ACTIVE      = 0x00000020; // 行情追踪委托的激活状态
        const TRACE_FIRE        = 0x00000040; // 行情追踪委托的触发状态
        const TRACE_AT_MAX      = 0x00000080; // 设定此标志以跟踪最大值的回调。不设定此标志以跟踪最小值的回调
        const FEE_IN_TPCOIN     = 0x00000100; // 是否允许第三币种支付手续费
        const HTTPCALL          = 0x08000000; // 测试
    }
}

/* 生效时间 */
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum TimeInForce {
    GoodTillCancel = 0, // 一直有效
    // ImmediateOrCancel = 1,   // 部分成交后剩余委托取消
    FillAndKill = 1,    // 部分成交后剩余委托取消
    FillOrKill = 2,     // 如果不能全部成交则取消委托(全部成交或者全部撤销)
}

impl Default for TimeInForce {
    fn default() -> TimeInForce {
        TimeInForce::GoodTillCancel
    }
}

/* 交易类型 */
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum TradeClass {
    Nil = 0,
    SPOT = 1,      // Spot Trading 现货交易
    FUTURE = 2,    // Future Trading 期货交易
    PERPETUAL = 3, // 永续
}

impl Default for TradeClass {
    fn default() -> TradeClass {
        TradeClass::Nil
    }
}

/* 合约标志 */
bitflags! {
    #[derive(Default, Serialize, Deserialize)]
    pub struct AssetFlags: u32 {
        const PRZ_INVERSE       = 0x00000001; // 例:盈亏 = 合约数量 * 乘数 * ( ( - 1/平仓价格) - ( - 1/开仓价格 ) ); 如果是正向合约: 盈亏 = 合约数量 * 乘数 * ( ( 1 * 平仓价格 ) - ( 1 * 开仓价格 ) )
        const DO_ADL            = 0x00000002; // 在必要的情况下，进行自动减仓操作。未使用
        const AUTO_SETTLE       = 0x00000004; // 是否自动结算
        const DENY_OPEN         = 0x00000008; // 禁止开仓
        const TRADE_STOPPED     = 0x00000010; // 停止交易
        const FEE_R_FOR_BUYSELL = 0x00000020; // 手续费率设定方法. 如果有此标志，则手续费设定的参数: FeeMkrR 表示的数据为 买方费率 FeeTkrR 表示的数据为卖方数据
        const ENABLE_MINING     = 0x00000040; // 激活挖矿系统
        const PRIVATE_OF_VP     = 0x00000080; // 私有交易对. (比如 VP1 的交易对，仅仅允许VP1的用户交易)
        const UPDATE_PRZ_LIMIT  = 0x00000100; // KNodelist更新时，更新价格区段
        const DATA_INVALID      = 0x00000200; // 数据失效
        const NO_MINING_BY_ASK  = 0x00000400; // 当启动了挖矿模式. 挖矿与卖操作无关
        const NO_MINING_BY_BID  = 0x00000800; // 当启动了挖矿模式. 挖矿与买操作无关
        const ENABLE_MULTI_POS  = 0x00001000; // 允许多仓位
    }
}

/* 手续费计算方法 */
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum FeeMethod {
    FM_IN_FROM_TO = 0,         // 收入货币中支付。对于买卖双方，使用不同的货币支付手续费
    FM_IN_FROM = 1,            // 使用购买行为中消费的币种为手续费
    FM_IN_FROM_TO_FEECOIN = 2, // 可以使用第三货币进行手续费抵扣。如果额度不足，则使用FROM_TO 的逻辑
    FM_IN_FROM_FEECOIN = 3,    // 可以使用第三货币进行手续费抵扣。如果额度不足，则使用FROM    的逻辑
}

impl Default for FeeMethod {
    fn default() -> FeeMethod {
        FeeMethod::FM_IN_FROM_TO
    }
}

/* 钱包标志 */
bitflags! {
    #[derive(Default, Serialize, Deserialize)]
    pub struct WltFlags: u32 {
        const SKIP_TO_ADL       = 0x00000001; // 立即ADL如果爆仓
        const DENY_DEPO         = 0x00000002; // 禁止入金
        const DENY_WDRW         = 0x00000004; // 禁止出金
        const DENY_ORDER        = 0x00000008; // 禁止下单
    }
}

/* 钱包状态 */
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum WltStatus {
    Nil = 0,
    NOT_ACTIVED = 1, // 尚未激活
    NORMAL = 2,      // 正常状态
    LIQUIDATION = 3, // 强平状态
    TAKEN_OVER = 4,  // 接管
}

impl Default for WltStatus {
    fn default() -> WltStatus {
        WltStatus::Nil
    }
}

/* 合约、交易对的状态 */
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum MkStatus {
    Nil = 0,
    MS_NORMAL = 1, // 正常运行
    MS_ADL = 2,    // 自动减仓
    MS_PAUSE = 3,  // 暂停
    MS_CLOSED = 4, // 交易对已经关闭
}

impl Default for MkStatus {
    fn default() -> MkStatus {
        MkStatus::Nil
    }
}

/* 钱包操作 */
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum WltOp {
    Nil = 0,
    DEPOSIT = 1,        // 存钱
    WITHDRAW = 2,       // 取钱
    PNL = 3,            // 已实现盈亏
    SPOT = 4,           // 现货交易
    TRAN_1_TO_MANY = 5, // 一账户 与 多账户 进行操作
    PNLISO = 6,         // 逐仓 已实现盈亏
    GIFT = 7,           // 礼金
    QUERY = 8,          // 查询
}

impl Default for WltOp {
    fn default() -> WltOp {
        WltOp::Nil
    }
}

/* 错误代码 */
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ErrorCode {
    NOERROR = 0,                    // 没有错误
    GENERAL = 1,                    // 数据错误
    DATA = 2,                       // 数据错误
    NOT_IMPLEMENTED = 3,            // 服务器未实现
    NO_MARGIN = 4,                  // 保证金不足
    FATAL = 5,                      // 致命错误
    NOT_FOUND = 6,                  // 未找到
    UNKNOWN_DIR = 7,                // 未知的委托方向
    INVALID_CODE = 8,               // 操作码错误
    EXISTS = 9,                     // 已存在
    NOT_FOUND_ORD = 10,             // 未找到委托
    PRZ_INVALID = 11,               // 价格错误
    EXPIRED = 12,                   // 已过期
    NOT_SUFFICIENT = 13,            // 资金不足
    WILLFILL = 14,                  // 对于PostOnly，本委托会成交
    EXECUTE_FAIL = 15,              // 对FillOrKill委托，这表示执行撮合失败
    EXCEED_LIMIT_MINVAL = 16,       // 超过限制
    ORDQTY_TOO_BIG_TOO_SMALL = 17,  // 委托价值太小
    EXCEED_LIMIT_PRZ_QTY = 18,      // 价格或者数量超出限制
    DENYOPEN_BY_POS = 19,           // 仓位超出限制
    DENYOPEN_BY_RD = 20,            // 禁止开仓
    TRADE_STOPED = 21,              // 交易暂停
    EXCEED_PRZ_LIQ = 22,            // 超过强平价格
    TOO_MANY_ORDER = 23,            // 太多的委托
    DENYOPEN_BY_TIME = 24,          // 超出开仓时间限制
    MD5_INVALID = 25,               // MD5签名验证错误
    RATELIMIT = 26,                 // 限速
    USER_CANCELED = 27,             // 用户撤销
    NOT_FOUND_WLT = 28,             // 无法找到钱包
    NOT_FOUND_MKT = 29,             // 未找到交易对
    EXCEED_MAXORDVAL = 30,          // 超过最大委托价值
    WILL_LIQUIDATE = 31,            // 将导致爆仓、强平
    NOT_IN_TRADE_PERIOD = 32,       // 非交易时间
    EXCEED_RAISE_FALL_R = 33,       // 超过涨跌停价格闲置
    PRZ_TOO_LOW = 34,               // 超出最小价格闲置
    EXCEED_TRADE_VOL = 35,          // 超出交易量限制
    EXCEED_TRADE_COUNT = 36,        // 超出交易次数闲置
    EXCEED_ASK_BID_PRZ_RATE = 37,   // 委托价格 超过盘口最新价格偏离
    EXCEED_TRDSUM = 39,             // TRDSUM限制
    OVERLOAD = 40,                  // OVERLOAD
    TOO_MANY_POS = 41,              // TOO_MANY_POS
    CHANNEL_BUSY = 42,              // CHANNEL_BUSY
    NO_DEFAULT_RISKLIMIT = 64,      // 没有指定风险限额
    TIMEOUT = 99,                   // 执行超时
}

impl Default for ErrorCode {
    fn default() -> ErrorCode {
        ErrorCode::NOERROR
    }
}

/* 委托单 */
#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Ord {
    #[serde(skip_serializing_if = "Option::is_none")] pub UId: Option<String>,        // 用户Id
    #[serde(skip_serializing_if = "Option::is_none")] pub AId: Option<String>,        // 账户Id
    #[serde(skip_serializing_if = "Option::is_none")] pub Sym: Option<String>,        // 交易对, 比如 BTC.USDT, ETH/USDT, BTC1912 等
    #[serde(skip_serializing_if = "Option::is_none")] pub WId: Option<String>,        // 钱包ID
    #[serde(skip_serializing_if = "Option::is_none")] pub OrdId: Option<String>,      // 服务器端为其分配的ID
    #[serde(skip_serializing_if = "Option::is_none")] pub COrdId: Option<String>,     // 客户端为其分配的ID
    #[serde(skip_serializing_if = "Option::is_none")] pub Dir: Option<OrderDir>,      // 委单方向 1=买/-1=卖,  0:Invalid, 1:BID/BUY, -1:ASK/SELL
    #[serde(skip_serializing_if = "Option::is_none")] pub OType: Option<OfferType>,   // 报价类型
    #[serde(skip_serializing_if = "Option::is_none")] pub Prz: Option<Decimal>,       // 价格
    #[serde(skip_serializing_if = "Option::is_none")] pub Qty: Option<Decimal>,       // 数量
    #[serde(skip_serializing_if = "Option::is_none")] pub QtyDsp: Option<Decimal>,    // 显示数量。如果为0,则显示全部Qty
    #[serde(skip_serializing_if = "Option::is_none")] pub Tif: Option<TimeInForce>,   // 有效期
    #[serde(skip_serializing_if = "Option::is_none")] pub OrdFlag: Option<u32>,       // 委托标志位,具体定义参考 OrdFlags;
    #[serde(skip_serializing_if = "Option::is_none")] pub Via: Option<OrderVia>,      // 来源
    #[serde(skip_serializing_if = "Option::is_none")] pub At: Option<i64>,            // 下单时间戳.单位:毫秒
    #[serde(skip_serializing_if = "Option::is_none")] pub Upd: Option<i64>,           // 更新时间戳.单位:毫秒
    #[serde(skip_serializing_if = "Option::is_none")] pub Until: Option<i64>,         // 有效期: 毫秒。绝对时间
    #[serde(skip_serializing_if = "Option::is_none")] pub PrzChg: Option<i32>,        // 市价委托的最大档位(当撮合进行匹配的时候，会从Orderbook依档位进行)
    #[serde(skip_serializing_if = "Option::is_none")] pub Frz: Option<Decimal>,       // 冻结金额
    #[serde(skip_serializing_if = "Option::is_none")] pub ErrCode: Option<i32>,       // 错误代码
    #[serde(skip_serializing_if = "Option::is_none")] pub ErrTxt: Option<String>,     // 错误文本
    #[serde(skip_serializing_if = "Option::is_none")] pub Status: Option<OrderStatus>,// 状态
    #[serde(skip_serializing_if = "Option::is_none")] pub QtyF: Option<Decimal>,      // 已成交 Qty Filled
    #[serde(skip_serializing_if = "Option::is_none")] pub PrzF: Option<Decimal>,      // 已成交的平均价格 Prz Filled
    #[serde(skip_serializing_if = "Option::is_none")] pub Val: Option<Decimal>,       // 合约价值, 对于PRZ_INVERSE的合约: - Dir * Qty / Prz; 对于正向合约 Dir * Qty * Prz
    #[serde(skip_serializing_if = "Option::is_none")] pub PId: Option<String>,        // 仓位Id,如果指定了仓位Id,则本委托导致的的仓位变化，为修改指定的仓位
    /* ------------------------------------------------------------ */
    #[serde(skip_serializing_if = "Option::is_none")] pub StopBy: Option<StopBy>,     // 判断依据
    #[serde(skip_serializing_if = "Option::is_none")] pub StopPrz: Option<Decimal>,   // 止损价格,止盈价格
    #[serde(skip_serializing_if = "Option::is_none")] pub TraceRR: Option<f64>,       // 追踪委托中，回调的比率. Reverse Ratio. 小数。
    #[serde(skip_serializing_if = "Option::is_none")] pub TraceMin: Option<f64>,      // 追踪的Min
    #[serde(skip_serializing_if = "Option::is_none")] pub TraceMax: Option<f64>,      // 追踪的Max
    /* ------------------------------------------------------------ */
    #[serde(skip_serializing_if = "Option::is_none")] pub MM: Option<f64>,            // 委托保证金 Mgn Initial + 佣金
    #[serde(skip_serializing_if = "Option::is_none")] pub FeeEst: Option<f64>,        // 预估的手续费: 按照手续费计算
    #[serde(skip_serializing_if = "Option::is_none")] pub UPNLEst: Option<f64>,       // 预估的UPNL Predicatee
    #[serde(skip_serializing_if = "Option::is_none")] pub VP: Option<i64>,            // 虚拟平台ID, 子交易所编号
}

/* 持仓 */
#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Position {
    #[serde(skip_serializing_if = "Option::is_none")] pub UId: Option<String>,        // 用户Id
    #[serde(skip_serializing_if = "Option::is_none")] pub PId: Option<String>,        // 仓位Id
    #[serde(skip_serializing_if = "Option::is_none")] pub AId: Option<String>,        // 账户Id
    #[serde(skip_serializing_if = "Option::is_none")] pub Sym: Option<String>,        // 交易对, 比如 BTC.USDT, ETH/USDT, BTC1912 等
    #[serde(skip_serializing_if = "Option::is_none")] pub WId: Option<String>,        // 钱包ID
    #[serde(skip_serializing_if = "Option::is_none")] pub Sz: Option<Decimal>,        // 仓位(正数为多仓，负数为空仓)
    #[serde(skip_serializing_if = "Option::is_none")] pub PrzIni: Option<Decimal>,    // 开仓平均价格
    #[serde(skip_serializing_if = "Option::is_none")] pub RPNL: Option<f64>,          // 已实现盈亏
    #[serde(skip_serializing_if = "Option::is_none")] pub Lever: Option<f64>,         // 杠杆
    #[serde(skip_serializing_if = "Option::is_none")] pub MgnISO: Option<Decimal>,    // 逐仓下仓位保证金
    #[serde(skip_serializing_if = "Option::is_none")] pub PNLISO: Option<Decimal>,    // 逐仓下已实现盈亏
    /** 下面是动态数据 */
    #[serde(skip_serializing_if = "Option::is_none")] pub LeverMax: Option<f64>,      // 最大杠杆
    #[serde(skip_serializing_if = "Option::is_none")] pub MMR: Option<f64>,           // 有效MMR
    #[serde(skip_serializing_if = "Option::is_none")] pub MIR: Option<f64>,           // 有效MIR
    #[serde(skip_serializing_if = "Option::is_none")] pub Val: Option<f64>,           // 计算值：价值,仓位现时的名义价值，受到标记价格价格的影响
    #[serde(skip_serializing_if = "Option::is_none")] pub Flg: Option<u32>,           // 标志, 0:NONE, 1:IS_MASTER(缺省仓位), 2:ENABLE_STOPP(启用至盈价), 4:ENABLE_STOPL(启用至损价)
    #[serde(skip_serializing_if = "Option::is_none")] pub MMnF: Option<f64>,          // 保证金，被仓位使用并锁定的保证金
    #[serde(skip_serializing_if = "Option::is_none")] pub MI: Option<f64>,            // 委托保证金 = 计算自已有委单 + 平仓佣金 + 开仓佣金 Mgn Initial
    #[serde(skip_serializing_if = "Option::is_none")] pub UPNL: Option<f64>,          // 计算值：未实现盈亏 PNL==  Profit And Loss
    #[serde(skip_serializing_if = "Option::is_none")] pub PrzLiq: Option<f64>,        // 计算值: 强平价格 亏光当前保证金的 (如果是多仓，并且标记价格低于PrzLiq,则会被强制平仓。/如果是空仓,并缺标记价格高于PrzLiq，则会被强制平仓
    #[serde(skip_serializing_if = "Option::is_none")] pub PrzBr: Option<f64>,         // 计算值: 破产价格 BandRuptcy
    #[serde(skip_serializing_if = "Option::is_none")] pub FeeEst: Option<f64>,        // 预估的平仓费
    /* ------------------------------------------------------------ */
    #[serde(skip_serializing_if = "Option::is_none")] pub StopPBy: Option<StopBy>,    // 止盈方法
    #[serde(skip_serializing_if = "Option::is_none")] pub StopP: Option<f64>,         // 止盈价
    #[serde(skip_serializing_if = "Option::is_none")] pub StopLBy: Option<StopBy>,    // 止损方法
    #[serde(skip_serializing_if = "Option::is_none")] pub StopL: Option<f64>,         // 止损价
    /* ------------------------------------------------------------ */
    #[serde(skip_serializing_if = "Option::is_none")] pub ROE: Option<f64>,           // 净资产收益率
    #[serde(skip_serializing_if = "Option::is_none")] pub ADLIdx: Option<f64>,        // ADLIdx, 这个是用来排序ADL的
    #[serde(skip_serializing_if = "Option::is_none")] pub ADLLight: Option<i32>,      // ADL红绿灯
}

/* 钱包 */
#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Wlt {
    #[serde(skip_serializing_if = "Option::is_none")] pub UId: Option<String>,        // 用户Id
    #[serde(skip_serializing_if = "Option::is_none")] pub AId: Option<String>,        // 账户Id
    #[serde(skip_serializing_if = "Option::is_none")] pub Coin: Option<String>,       // 货币类型
    #[serde(skip_serializing_if = "Option::is_none")] pub WId: Option<String>,        // 钱包索引
    #[serde(skip_serializing_if = "Option::is_none")] pub Depo: Option<Decimal>,      // 入金金额
    #[serde(skip_serializing_if = "Option::is_none")] pub WDrw: Option<Decimal>,      // 出金金额
    #[serde(skip_serializing_if = "Option::is_none")] pub PNL: Option<Decimal>,       // 已实现盈亏
    #[serde(skip_serializing_if = "Option::is_none")] pub Frz: Option<Decimal>,       // 冻结金额
    /** 下面是统计值 */
    #[serde(skip_serializing_if = "Option::is_none")] pub UPNL: Option<f64>,          // 未实现盈亏：根据持仓情况、标记价格 刷新
    #[serde(skip_serializing_if = "Option::is_none")] pub MI: Option<f64>,            // 委托保证金 = 计算自已有委单 + 平仓佣金 + 开仓佣金 Mgn Initial
    #[serde(skip_serializing_if = "Option::is_none")] pub MM: Option<f64>,            // 仓位保证金 + 平仓佣金 Mgn Maintaince
    #[serde(skip_serializing_if = "Option::is_none")] pub RD: Option<f64>,            // 风险度 // Risk Degree.
    #[serde(skip_serializing_if = "Option::is_none")] pub Wdrawable: Option<f64>,     // 可取余额 . 定时刷新
    #[serde(skip_serializing_if = "Option::is_none")] pub Spot: Option<Decimal>,      // 现货交易出入金
    #[serde(skip_serializing_if = "Option::is_none")] pub Gift: Option<Decimal>,      // 赠送金额 不允许取出
    #[serde(skip_serializing_if = "Option::is_none")] pub PNLG: Option<Decimal>,      // Gift不为0的时候
    #[serde(skip_serializing_if = "Option::is_none")] pub Status: Option<WltStatus>,  // 账户状态
    #[serde(skip_serializing_if = "Option::is_none")] pub Flg: Option<u32>,           // WltFlags 钱包标志位
    #[serde(skip_serializing_if = "Option::is_none")] pub VP: Option<i64>,            // 虚拟平台ID, 子交易所编号
}

/* 资金历史 */
#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct WltLog {
    #[serde(skip_serializing_if = "Option::is_none")] pub UId: Option<String>,        // 用户Id
    #[serde(skip_serializing_if = "Option::is_none")] pub AId: Option<String>,        // 账户Id
    #[serde(skip_serializing_if = "Option::is_none")] pub Seq: Option<String>,        // 序列号
    #[serde(skip_serializing_if = "Option::is_none")] pub Coin: Option<String>,       // 货币类型
    #[serde(skip_serializing_if = "Option::is_none")] pub WId: Option<String>,        // 钱包Id
    #[serde(skip_serializing_if = "Option::is_none")] pub Qty: Option<Decimal>,       // 数量
    #[serde(skip_serializing_if = "Option::is_none")] pub Fee: Option<Decimal>,       // 手续费
    #[serde(skip_serializing_if = "Option::is_none")] pub Peer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub WalBal: Option<Decimal>,    // 余额
    #[serde(skip_serializing_if = "Option::is_none")] pub At: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")] pub Op: Option<WltOp>,
    #[serde(skip_serializing_if = "Option::is_none")] pub Via: Option<OrderVia>,
    #[serde(skip_serializing_if = "Option::is_none")] pub Info: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub ErrCode: Option<ErrorCode>,
    #[serde(skip_serializing_if = "Option::is_none")] pub Stat: Option<OrderStatus>,
    #[serde(skip_serializing_if = "Option::is_none")] pub VP: Option<i64>,            // 虚拟平台ID, 子交易所编号
}

/* 成交记录 */
#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct TrdRec {
    #[serde(skip_serializing_if = "Option::is_none")] pub UId: Option<String>,        // 用户Id
    #[serde(skip_serializing_if = "Option::is_none")] pub AId: Option<String>,        // 账户Id
    #[serde(skip_serializing_if = "Option::is_none")] pub Sym: Option<String>,        // 交易对符号
    #[serde(skip_serializing_if = "Option::is_none")] pub WId: Option<String>,        // 钱包Id
    #[serde(skip_serializing_if = "Option::is_none")] pub MatchId: Option<String>,    // 撮合ID
    #[serde(skip_serializing_if = "Option::is_none")] pub OrdId: Option<String>,      // 委托单ID
    #[serde(skip_serializing_if = "Option::is_none")] pub Sz: Option<Decimal>,        // 数量
    #[serde(skip_serializing_if = "Option::is_none")] pub Prz: Option<Decimal>,       // 价格
    #[serde(skip_serializing_if = "Option::is_none")] pub Fee: Option<Decimal>,       // 手续费
    #[serde(skip_serializing_if = "Option::is_none")] pub FeeCoin: Option<String>,    // 手续费币种
    #[serde(skip_serializing_if = "Option::is_none")] pub At: Option<i64>,            // 时间戳,毫秒
    #[serde(skip_serializing_if = "Option::is_none")] pub Via: Option<OrderVia>,      // 委托来源
    #[serde(skip_serializing_if = "Option::is_none")] pub PAId: Option<String>,       // 对手账户Id
    #[serde(skip_serializing_if = "Option::is_none")] pub Liq: Option<f64>,           // 强平价格
    #[serde(skip_serializing_if = "Option::is_none")] pub Br: Option<f64>,            // 破产价格
    #[serde(skip_serializing_if = "Option::is_none")] pub Lvr: Option<f64>,           // Lever
    #[serde(skip_serializing_if = "Option::is_none")] pub PrzM: Option<f64>,          // 标记价格
    #[serde(skip_serializing_if = "Option::is_none")] pub PId: Option<String>,        // 仓位Id
    #[serde(skip_serializing_if = "Option::is_none")] pub GrossVal: Option<f64>,      // 本成交单的价值
    // pub HomeNotional: Option<f64>,
    // pub foreignNotional: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")] pub Gift: Option<f64>,          // 赠金
    /* ------------------------------------------------------------ */
    /** 下面的数据，来自Trdsum */
    #[serde(skip_serializing_if = "Option::is_none")] pub BAvg: Option<f64>,          // 平均买入价
    #[serde(skip_serializing_if = "Option::is_none")] pub NBid: Option<f64>,          // 计算平均值的买入量
    #[serde(skip_serializing_if = "Option::is_none")] pub AAvg: Option<f64>,          // 平均卖出价
    #[serde(skip_serializing_if = "Option::is_none")] pub NAsk: Option<f64>,          // 计算平均值的卖出量
    #[serde(skip_serializing_if = "Option::is_none")] pub SzBid: Option<f64>,         // 统计周期内买入量
    #[serde(skip_serializing_if = "Option::is_none")] pub SzAsk: Option<f64>,         // 统计周期内卖出量
    #[serde(skip_serializing_if = "Option::is_none")] pub NumBid: Option<i64>,        // 统计周期内买入次数
    #[serde(skip_serializing_if = "Option::is_none")] pub NumAsk: Option<i64>,        // 统计周期内卖出次数
    /* ------------------------------------------------------------ */
    #[serde(skip_serializing_if = "Option::is_none")] pub MPL: Option<i64>,          // 算力等级
    #[serde(skip_serializing_if = "Option::is_none")] pub MPB: Option<f64>,          // 买入算力 Mine Power for Bid
    #[serde(skip_serializing_if = "Option::is_none")] pub MPA: Option<f64>,          // 卖出算力 Mine Power for Ask
    #[serde(skip_serializing_if = "Option::is_none")] pub MPS: Option<f64>,          // 算力相关量. 可能并不会等于 Sz
    /* ------------------------------------------------------------ */
    #[serde(skip_serializing_if = "Option::is_none")] pub Ext: Option<String>,       // 扩展字段
    /* ------------------------------------------------------------ */
    /** 下面字段用来描述开平仓和收益的 */
    #[serde(skip_serializing_if = "Option::is_none")] pub PrzIC: Option<f64>,        // 平仓操作的开仓价
    #[serde(skip_serializing_if = "Option::is_none")] pub SzCls: Option<f64>,        // 平仓数量
    #[serde(skip_serializing_if = "Option::is_none")] pub PnlCls: Option<f64>,       // 平仓收益
    #[serde(skip_serializing_if = "Option::is_none")] pub PrzIO: Option<f64>,        // 仓位的最终开仓价格
    #[serde(skip_serializing_if = "Option::is_none")] pub SzOpn: Option<f64>,        // 仓位的最终值
    /* ------------------------------------------------------------ */
    #[serde(skip_serializing_if = "Option::is_none")] pub VP: Option<i64>,           // 虚拟平台ID
}

/* 交易对 */
#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct AssetD {
    #[serde(skip_serializing_if = "Option::is_none")] pub Sym: Option<String>,          // 交易对符号, BTC.USDT, ETH/ETH ...
    #[serde(skip_serializing_if = "Option::is_none")] pub Beg: Option<i64>,             // 开始时间
    #[serde(skip_serializing_if = "Option::is_none")] pub Expire: Option<i64>,          // 到期日期 永续
    /* ------------------------------------------------------------ */
    #[serde(skip_serializing_if = "Option::is_none")] pub PrzMaxChg: Option<i32>,       // 市价委托的撮合的最多次数。比如5
    #[serde(skip_serializing_if = "Option::is_none")] pub PrzMinInc: Option<Decimal>,   // 最小的价格变化 0.5 USD
    #[serde(skip_serializing_if = "Option::is_none")] pub PrzMax: Option<Decimal>,      // 最大委托价格 1,000,000
    #[serde(skip_serializing_if = "Option::is_none")] pub OrderMaxQty: Option<Decimal>, // 最大委托数量 10,000,000
    #[serde(skip_serializing_if = "Option::is_none")] pub LotSz: Option<Decimal>,       // 最小合约数量  这个就是每次买卖的合约数量必须是LotSz的倍数。
    /* 保证金计算相关参数 开始 */
    #[serde(skip_serializing_if = "Option::is_none")] pub PrzM: Option<Decimal>,        // 标记价格 8103.14
    #[serde(skip_serializing_if = "Option::is_none")] pub MIR: Option<Decimal>,         // 起始保证金 1.00% + 开仓佣金 + 平仓佣金 Mgn Initial Ratio
    #[serde(skip_serializing_if = "Option::is_none")] pub MMR: Option<Decimal>,         // 维持保证金  0.50% + 平仓佣金 + 资金费率 Mgn Maintaince Ratio
    #[serde(skip_serializing_if = "Option::is_none")] pub PrzMBiasR: Option<f64>,       // 当前价格的最大偏离率
    /* 保证金计算相关参数 结束 */
    /* 统计信息 */
    #[serde(skip_serializing_if = "Option::is_none")] pub PrzLatest: Option<Decimal>,   // 最新成交价格
    #[serde(skip_serializing_if = "Option::is_none")] pub DirLatest: Option<OrderDir>,  // 最新成交的方向
    // 最新的成交方向
    #[serde(skip_serializing_if = "Option::is_none")] pub TotalVol: Option<f64>,        // 总交易量 30,585,913,058
    #[serde(skip_serializing_if = "Option::is_none")] pub OpenInterest: Option<f64>,    // 持仓量  99,192,762
    #[serde(skip_serializing_if = "Option::is_none")] pub Turnover: Option<f64>,        // 总成交额 26,293.1141 XBT
    #[serde(skip_serializing_if = "Option::is_none")] pub PrzIndex: Option<Decimal>,    // 指数价格
    #[serde(skip_serializing_if = "Option::is_none")] pub AssetSz: Option<i64>,         // 合约大小
    #[serde(skip_serializing_if = "Option::is_none")] pub PosLmtStart: Option<i64>,     // 当总开仓到达这个数字，启动个人开仓率限制。
    #[serde(skip_serializing_if = "Option::is_none")] pub PrzRFMin: Option<f64>,        // 当前涨跌价格范围 Prz Rise Fall Range
    #[serde(skip_serializing_if = "Option::is_none")] pub PrzRFMax: Option<f64>,        // 当前涨跌价格范围最大值
    /* 佣金费率 */
    #[serde(skip_serializing_if = "Option::is_none")] pub FeeMkrR: Option<Decimal>,     // 提供流动性的费率 FeeMkrR
    #[serde(skip_serializing_if = "Option::is_none")] pub FeeTkrR: Option<Decimal>,     // 消耗流动性的费率
    #[serde(skip_serializing_if = "Option::is_none")] pub Mult: Option<Decimal>,        // Order中，Qty必须是Mult的倍数
    #[serde(skip_serializing_if = "Option::is_none")] pub FromC: Option<String>,        // 从什么货币 购买行为消耗的货币符号
    #[serde(skip_serializing_if = "Option::is_none")] pub ToC: Option<String>,          // 兑换为 什么货币  购买行为得到的货币符号
    #[serde(skip_serializing_if = "Option::is_none")] pub PrzCls: Option<f64>,          // 最近一个K线周期的收盘价。如果某K线指标被配置并指定了更新PrzCls,则此价格会得到更新
    /* ------------------------------------------------------------ */
    #[serde(skip_serializing_if = "Option::is_none")] pub TrdCls: Option<TradeClass>,   // 交易类型, 期货、现货
    #[serde(skip_serializing_if = "Option::is_none")] pub MkSt: Option<MkStatus>,       // 市场状态
    #[serde(skip_serializing_if = "Option::is_none")] pub Flag: Option<u32>,            // 交易对标志位, 具体定义参考 AssetFlags
    // 标志, 正向报价，反向报价
    #[serde(skip_serializing_if = "Option::is_none")] pub SettleCoin: Option<String>,         // 结算货币
    #[serde(skip_serializing_if = "Option::is_none")] pub QuoteCoin: Option<String>,          // 报价货币
    #[serde(skip_serializing_if = "Option::is_none")] pub SettleR: Option<Decimal>,           // 结算费率
    #[serde(skip_serializing_if = "Option::is_none")] pub DenyOpenAfter: Option<i64>,         // 时间节点：当越过了DenyOpenAfter后，不允许开新仓
    #[serde(skip_serializing_if = "Option::is_none")] pub OrderMinQty: Option<Decimal>,       // 最小委托数量
    /** 永续合约专属数据 */
    #[serde(skip_serializing_if = "Option::is_none")] pub InterestBaseSym: Option<String>,    // 基础货币利率符号
    #[serde(skip_serializing_if = "Option::is_none")] pub InterestQuoteSym: Option<String>,   // 计价货币利率符号
    #[serde(skip_serializing_if = "Option::is_none")] pub FundingPremiumSym: Option<String>,  // 资金费用溢价符号
    /** 资金费率 */
    #[serde(skip_serializing_if = "Option::is_none")] pub FundingLongR: Option<f64>,          // 多仓资金费率
    #[serde(skip_serializing_if = "Option::is_none")] pub FundingShortR: Option<f64>,         // 空仓资金费率
    #[serde(skip_serializing_if = "Option::is_none")] pub FundingInterval: Option<u32>,       // 资金费用收取间隔 秒
    #[serde(skip_serializing_if = "Option::is_none")] pub FundingNext: Option<i64>,           // 下一个资金费率结算的时间, 时间戳 毫秒
    #[serde(skip_serializing_if = "Option::is_none")] pub FundingPredictedR: Option<f64>,     // 预测费率
    #[serde(skip_serializing_if = "Option::is_none")] pub FundingOffset: Option<i64>,         // 每日0点后的 FundingOffset 毫秒后 为第一个结算时间点
    #[serde(skip_serializing_if = "Option::is_none")] pub FundingTolerance: Option<f64>,      // 资金费率计算参数: 公差
    #[serde(skip_serializing_if = "Option::is_none")] pub FundingFeeR: Option<Decimal>,           // Funding结算佣金
    /* ------------------------------------------------------------ */
    #[serde(skip_serializing_if = "Option::is_none")] pub FundingAId: Option<String>,         // 资金结算佣金账户
    #[serde(skip_serializing_if = "Option::is_none")] pub InsurAIdLong: Option<String>,       // 多仓保险账户
    #[serde(skip_serializing_if = "Option::is_none")] pub ServeAId: Option<String>,           // 服务费账户
    #[serde(skip_serializing_if = "Option::is_none")] pub InsurAId: Option<String>,           // 空仓保险账户
    #[serde(skip_serializing_if = "Option::is_none")] pub Grp: Option<i64>,                   // 分组,当前主要前端UI显示是用来隐藏或分类用;
}

/* 交易对扩展属性 */
#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct V2AssetCfg {
    #[serde(skip_serializing_if = "Option::is_none")] pub Sym: Option<String>,        // 符号, BTC.USDT, ETH/ETH ...
    #[serde(skip_serializing_if = "Option::is_none")] pub FM: Option<FeeMethod>,      // 手续费计费方法
    #[serde(skip_serializing_if = "Option::is_none")] pub FeeCoin: Option<String>,    // 手续费币种
    #[serde(skip_serializing_if = "Option::is_none")] pub FeeDiscR: Option<Decimal>,  // 折扣率
    #[serde(skip_serializing_if = "Option::is_none")] pub OnAt: Option<u64>,          // 开放交易时间 (日内,毫秒)
    #[serde(skip_serializing_if = "Option::is_none")] pub OffAt: Option<u64>,         // 关闭交易时间 (日内,毫秒)
    #[serde(skip_serializing_if = "Option::is_none")] pub RiseR: Option<i64>,         // 价格涨价幅度 万分比 * 10000
    #[serde(skip_serializing_if = "Option::is_none")] pub FallR: Option<i64>,         // 价格跌价幅度 万分比 * 10000
    #[serde(skip_serializing_if = "Option::is_none")] pub PrzMin: Option<f64>,        // 最小价格
    #[serde(skip_serializing_if = "Option::is_none")] pub LmtBid: Option<f64>,        // 买入量
    #[serde(skip_serializing_if = "Option::is_none")] pub LmtAsk: Option<f64>,        // 卖出量
    #[serde(skip_serializing_if = "Option::is_none")] pub LmtBidAsk: Option<f64>,     // 买入卖出总量
    #[serde(skip_serializing_if = "Option::is_none")] pub BidPrzR: Option<f64>,       // 委托的买价偏离盘口比例(小数)
    #[serde(skip_serializing_if = "Option::is_none")] pub AskPrzR: Option<f64>,       // 委托的买价偏离盘口比例(小数)
    #[serde(skip_serializing_if = "Option::is_none")] pub LmtNetAsk: Option<f64>,     // 每统计周期 净卖量。如果为0，则表示不进行检查
    #[serde(skip_serializing_if = "Option::is_none")] pub SumAt: Option<u64>,         // 从0点开始，在每天的什么时间，开始重置统计值(绝对时间,毫秒)
    #[serde(skip_serializing_if = "Option::is_none")] pub SumInterval: Option<u64>,   // 重置间隔
    #[serde(skip_serializing_if = "Option::is_none")] pub SumResetNext: Option<u64>,  // 下次重置
    #[serde(skip_serializing_if = "Option::is_none")] pub SzForAvg: Option<f64>,      // 求用户的最近的买入价格的量
    #[serde(skip_serializing_if = "Option::is_none")] pub FeeMkrMin: Option<Decimal>,   // Maker最低手续费
    #[serde(skip_serializing_if = "Option::is_none")] pub FeeTkrMin: Option<Decimal>,   // Taker最低手续费
    /** 下面是挖矿相关设定 */
    #[serde(skip_serializing_if = "Option::is_none")] pub SzMaxFM: Option<f64>,       // 每日有挖矿算力的交易量
    #[serde(skip_serializing_if = "Option::is_none")] pub NumMaxFM: Option<f64>,      // 每日有挖矿算力的交易次数
    #[serde(skip_serializing_if = "Option::is_none")] pub ExpRatio: Option<f64>,      // 涨经验的交易量完成率.当交易量达到 SzMaxFM * ExpRatio Exp ++
    #[serde(skip_serializing_if = "Option::is_none")] pub ExpMax: Option<i64>,        // 最大Exp
    #[serde(skip_serializing_if = "Option::is_none")] pub Flag: Option<u32>,          // AssetFlags 标志位
    // /** 一些通用参数 */
    // #[serde(skip_serializing_if = "Option::is_none")] pub F0: Option<f64>,
    // #[serde(skip_serializing_if = "Option::is_none")] pub F1: Option<f64>,
    // #[serde(skip_serializing_if = "Option::is_none")] pub F2: Option<f64>,
    // #[serde(skip_serializing_if = "Option::is_none")] pub F3: Option<f64>,
    // #[serde(skip_serializing_if = "Option::is_none")] pub F4: Option<f64>,
    // #[serde(skip_serializing_if = "Option::is_none")] pub F5: Option<f64>,
    // #[serde(skip_serializing_if = "Option::is_none")] pub F6: Option<f64>,
    // #[serde(skip_serializing_if = "Option::is_none")] pub F7: Option<f64>,
    // #[serde(skip_serializing_if = "Option::is_none")] pub F8: Option<f64>,
    // #[serde(skip_serializing_if = "Option::is_none")] pub F9: Option<f64>,
    // #[serde(skip_serializing_if = "Option::is_none")] pub I0: Option<i64>,
    // #[serde(skip_serializing_if = "Option::is_none")] pub I1: Option<i64>,
    // #[serde(skip_serializing_if = "Option::is_none")] pub I2: Option<i64>,
    // #[serde(skip_serializing_if = "Option::is_none")] pub I3: Option<i64>,
}

/* 风险限额定义 */
#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct RiskLimitDef {
    #[serde(skip_serializing_if = "Option::is_none")] pub Name: Option<String>,       // 本配置的名称
    #[serde(skip_serializing_if = "Option::is_none")] pub Sym: Option<String>,        // Symbol 交易对。或特定的名字，比如 XBTUSD_01,XBTUSD_99
    #[serde(skip_serializing_if = "Option::is_none")] pub Base: Option<f64>,          // Base Risk Limit 当 Pos       Val < Base 的时候
    #[serde(skip_serializing_if = "Option::is_none")] pub BaseMMR: Option<f64>,       // Base Maintenance Margin      Val < Base 的时候 MMR
    #[serde(skip_serializing_if = "Option::is_none")] pub BaseMIR: Option<f64>,       // Initial Margin               Val < Base 的时候 MIR
    #[serde(skip_serializing_if = "Option::is_none")] pub Step: Option<f64>,          // Step                         StepS = math.Ceil((Val - Base)/Step) 表示递增次数
    #[serde(skip_serializing_if = "Option::is_none")] pub StepMR: Option<f64>,        // StepM						每次递增的时候，MMR MIR 的增量
    #[serde(skip_serializing_if = "Option::is_none")] pub PosSzMax: Option<f64>,      // 最大持仓
    #[serde(skip_serializing_if = "Option::is_none")] pub StepIR: Option<f64>,        // StepIR						每次递增的时候，MIR 的增量
    #[serde(skip_serializing_if = "Option::is_none")] pub MaxOrdVal: Option<f64>,     // 单笔委托的最大价值
}


/* ------------------------------------------------------------ */
/* ------------------------------------------------------------ */
/* ------------------------------------------------------------ */

/** 资产中心-账户钱包对象, 通过用户中心查询时可以查询到的用户主钱包的信息 */
#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CcsMainWallet {
    #[serde(skip_serializing_if = "Option::is_none")] pub wid: Option<String>,          // 主键：资金账户id， uid+Wtype
    #[serde(skip_serializing_if = "Option::is_none")] pub uid: Option<String>,          // 用户账号uid
    #[serde(skip_serializing_if = "Option::is_none")] pub coin: Option<String>,         // 币种名称（如BTC/ETH等）
    #[serde(skip_serializing_if = "Option::is_none")] pub mainBal: Option<Decimal>,     // 主账户余额
    #[serde(skip_serializing_if = "Option::is_none")] pub mainLock: Option<Decimal>,    // 主账户锁币额度
    #[serde(skip_serializing_if = "Option::is_none")] pub otcBal: Option<Decimal>,      // otc法币账户余额
    #[serde(skip_serializing_if = "Option::is_none")] pub otcLock: Option<Decimal>,     // otc锁币额度
    #[serde(skip_serializing_if = "Option::is_none")] pub financeBal: Option<Decimal>,  // 理财额度
    #[serde(skip_serializing_if = "Option::is_none")] pub pawnBal: Option<Decimal>,     // 质押额度
    #[serde(skip_serializing_if = "Option::is_none")] pub creditNum: Option<Decimal>,   // 欠贷款额度【负】
    #[serde(skip_serializing_if = "Option::is_none")] pub wdLimit: Option<Decimal>,     // 提现限额
    #[serde(skip_serializing_if = "Option::is_none")] pub depositLock: Option<Decimal>, // 充值锁定（交易挖矿）
    #[serde(skip_serializing_if = "Option::is_none")] pub cTime: Option<i64>,           // 账户创建时间（秒）
    #[serde(skip_serializing_if = "Option::is_none")] pub updTime: Option<i64>,         // 账户创建时间（秒），每次更改刷新
    #[serde(skip_serializing_if = "Option::is_none")] pub flag: Option<i64>,            // 账户标记
    #[serde(skip_serializing_if = "Option::is_none")] pub memo: Option<String>,         // 账户备注
    #[serde(skip_serializing_if = "Option::is_none")] pub email: Option<String>,        // 账户名email
}

/** 通过用户中心查询资产时，返回的撮合的资产的数据结构, 本质上和 Wlt 的信息一致. */
#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CcsMatcherWallet {
    #[serde(skip_serializing_if = "Option::is_none")] pub wType: Option<String>,    // 币种
    #[serde(skip_serializing_if = "Option::is_none")] pub Num: Option<Decimal>,     // 金额（入金总额-出金总额）
    #[serde(skip_serializing_if = "Option::is_none")] pub PNL: Option<Decimal>,     // 已实现盈亏
    #[serde(skip_serializing_if = "Option::is_none")] pub Frz: Option<Decimal>,     // 冻结金额
    #[serde(skip_serializing_if = "Option::is_none")] pub UPNL: Option<f64>,        // 未实现盈亏：根据持仓情况、标记价格 刷新，统计值
    #[serde(skip_serializing_if = "Option::is_none")] pub PNLISO: Option<f64>,      // 逐仓下已实现盈亏
    #[serde(skip_serializing_if = "Option::is_none")] pub MI: Option<f64>,          // 委托保证金 = 计算自已有委单 + 平仓佣金 + 开仓佣金 Mgn Initial
    #[serde(skip_serializing_if = "Option::is_none")] pub MM: Option<f64>,          // 仓位保证金 + 平仓佣金 Mgn Maintaince
    #[serde(skip_serializing_if = "Option::is_none")] pub RD: Option<f64>,          // 风险度 // Risk Degree.
    #[serde(skip_serializing_if = "Option::is_none")] pub balance: Option<f64>,     // 计算得出的余额，仅当时有效
    #[serde(skip_serializing_if = "Option::is_none")] pub wdrawable: Option<f64>,   // 撮合计算出来的可取余额
    #[serde(skip_serializing_if = "Option::is_none")] pub Gift: Option<Decimal>,    // 合约赠金
    #[serde(skip_serializing_if = "Option::is_none")] pub PNLG: Option<Decimal>,    // 合约赠金盈亏
}
