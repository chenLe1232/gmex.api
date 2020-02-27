// GMEX-API 数据结构定义
//
// NOTE: 由于在 API/JSON 层上，如果某个参数没有传，并不是说这个参数不存在，而是这个参数使用默认值，
// 衡量下来还是觉得这里结构定义时不使用 Option 而是 用  is_default 来处理 JSON 序列号问题.
// 同时由于大量枚举类型的值都可能随时被增加，因此换成 struct(i32) 来适配。

#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_doc_comments)]

use super::Decimal;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::helper::is_default;

// #[macro_use]
// extern crate bitflags;

/** 委托单方向 买/卖 */
// #[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct OrderDir(pub i32);

pub const OrderDir_Nil: OrderDir = OrderDir(0);
pub const OrderDir_BID: OrderDir = OrderDir(1);
pub const OrderDir_BUY: OrderDir = OrderDir(1);
pub const OrderDir_ASK: OrderDir = OrderDir(-1);
pub const OrderDir_SELL: OrderDir = OrderDir(-1);

impl Default for OrderDir {
    fn default() -> Self {
        Self(0)
    }
}

impl From<i32> for OrderDir {
    fn from(other: i32) -> Self {
        Self(other)
    }
}

impl OrderDir {
    pub fn is_default(self) -> bool {
        self.0 == 0
    }
    pub fn is_nil(self) -> bool {
        self.0 == 0
    }
    pub fn is_bid(self) -> bool {
        self.0 == 1
    }
    pub fn is_buy(self) -> bool {
        self.is_bid()
    }
    pub fn is_ask(self) -> bool {
        self.0 == -1
    }
    pub fn is_sell(self) -> bool {
        self.is_ask()
    }
}

/** 来源: 用于 委托、交易、结算等 */
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct OrderVia(pub i32);

pub const OrderVia_Nil: OrderVia = OrderVia(0); // 空值
pub const OrderVia_WEB: OrderVia = OrderVia(1); // 通过 WEB 下单
pub const OrderVia_APP: OrderVia = OrderVia(2); // 通过 APP 下单
pub const OrderVia_API: OrderVia = OrderVia(3); // 通过 API-KEY 下单
pub const OrderVia_Liquidate: OrderVia = OrderVia(4); // 平仓 Liquidate
pub const OrderVia_ADLEngine: OrderVia = OrderVia(5); // ADL 减仓操作
pub const OrderVia_Settlement: OrderVia = OrderVia(6); // 结算
pub const OrderVia_Trade: OrderVia = OrderVia(7); // 交易
pub const OrderVia_Fee: OrderVia = OrderVia(8); // 手续费
pub const OrderVia_Depo: OrderVia = OrderVia(9); // 存
pub const OrderVia_Wdrw: OrderVia = OrderVia(10); // 取
pub const OrderVia_Funding: OrderVia = OrderVia(11); // Funding 资金费率
pub const OrderVia_Offer: OrderVia = OrderVia(12); // 配售
pub const OrderVia_TakeOver: OrderVia = OrderVia(13); // 接管
pub const OrderVia_PNLISO: OrderVia = OrderVia(14); // PNLISO 收入
pub const OrderVia_StopLP: OrderVia = OrderVia(15); // StopL StopP 盈亏
pub const OrderVia_XXXX: OrderVia = OrderVia(16); // XXXX 暂时没有定义
pub const OrderVia_Gift_Give: OrderVia = OrderVia(17); // 给予Gift 赠金
pub const OrderVia_Wlt_Settle: OrderVia = OrderVia(18); // 钱包结算
pub const OrderVia_Gift_Settle: OrderVia = OrderVia(19); // 赠金结算
pub const OrderVia_Planing: OrderVia = OrderVia(20); // 计划中
pub const OrderVia_ByPlan: OrderVia = OrderVia(21); // 已经执行

impl Default for OrderVia {
    fn default() -> Self {
        Self(0)
    }
}

impl From<i32> for OrderVia {
    fn from(other: i32) -> Self {
        Self(other)
    }
}

impl OrderVia {
    pub fn is_nil(self) -> bool {
        self.0 == 0
    }
}

/** 委托的状态 */
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct OrderStatus(pub i32);

pub const OrderStatus_Nil: OrderStatus = OrderStatus(0); // 空值
pub const OrderStatus_Queueing: OrderStatus = OrderStatus(1); // 正在排队
pub const OrderStatus_Matching: OrderStatus = OrderStatus(2); // 有效，已经进入order-book并等待撮合中
pub const OrderStatus_PostFail: OrderStatus = OrderStatus(3); // 提交失败
pub const OrderStatus_Executed: OrderStatus = OrderStatus(4); // 已执行

impl Default for OrderStatus {
    fn default() -> Self {
        Self(0)
    }
}

impl From<i32> for OrderStatus {
    fn from(other: i32) -> Self {
        Self(other)
    }
}

impl OrderStatus {
    pub fn is_nil(self) -> bool {
        self.0 == 0
    }
}

/** 报价方式 */
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct OfferType(pub i32);

pub const OfferType_Nil: OfferType = OfferType(0); // 空值
pub const OfferType_Limit: OfferType = OfferType(1); // 限价委单
pub const OfferType_Market: OfferType = OfferType(2); // 市价委单,匹配后转限价
pub const OfferType_StopLimit: OfferType = OfferType(3); // 限价止损/盈利
pub const OfferType_StopMarket: OfferType = OfferType(4); // 市价止损/盈利
pub const OfferType_TraceLimit: OfferType = OfferType(5); // 追踪 限价
pub const OfferType_TraceMarket: OfferType = OfferType(6); // 追踪 市价

impl Default for OfferType {
    fn default() -> Self {
        Self(0)
    }
}

impl From<i32> for OfferType {
    fn from(other: i32) -> Self {
        Self(other)
    }
}

impl OfferType {
    pub fn is_nil(self) -> bool {
        self.0 == 0
    }
}

/** 条件委托触发的判据 */
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct StopBy(pub i32);

pub const StopBy_PriceMark: StopBy = StopBy(0); // 标记价格
pub const StopBy_PriceLatest: StopBy = StopBy(1); // 最新成交
pub const StopBy_PriceIndex: StopBy = StopBy(2); // 指数价格

impl Default for StopBy {
    fn default() -> Self {
        Self(0)
    }
}

impl From<i32> for StopBy {
    fn from(other: i32) -> Self {
        Self(other)
    }
}

impl StopBy {
    pub fn is_nil(self) -> bool {
        self.0 == 0
    }
}

/** 交易指令的标志 */
bitflags! {
    #[derive(Default, Serialize, Deserialize)]
    pub struct OrderFlags: u32 {
        const POSTONLY          = 0x00000001; // 如果委托会立即成交，则不发送此委托
        const REDUCEONLY        = 0x00000002; // 如果委托会导致增加仓位，则不发送此委托
        const CLOSEONTRIGGER    = 0x00000004; // 触发后平仓 TODO 目前未实现
        const IF_GREATERTHAN    = 0x00000008; // 条件指定为 如果价格大于StopBy
        const IF_LESSTHAN       = 0x00000010; // 条件指定为 如果价格低于StopBy
        const TRACE_ACTIVE      = 0x00000020; // 行情追踪委托的激活状态
        const TRACE_FIRE        = 0x00000040; // 行情追踪委托的触发状态
        const TRACE_AT_MAX      = 0x00000080; // 设定此标志以跟踪最大值的回调。不设定此标志以跟踪最小值的回调
        const FEE_IN_TPCOIN     = 0x00000100; // 是否允许第三币种支付手续费
        const PRZ_OVER_LIQUIDATE= 0x00000200; // 超过强平价
        const MERGE_SAMEDIR     = 0x00000400; // 合并到特定的仓位
        const NO_TRADE          = 0x04000000; // NO_T
        const HTTPCALL          = 0x08000000; // 测试
    }
}

/** 生效时间 */
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct TimeInForce(pub i32);

pub const TimeInForce_GoodTillCancel: TimeInForce = TimeInForce(0); // 一直有效
pub const TimeInForce_FillAndKill: TimeInForce = TimeInForce(1); // 部分成交后剩余委托取消
pub const TimeInForce_ImmediateOrCancel: TimeInForce = TimeInForce(1); // 同 FOK
pub const TimeInForce_FillOrKill: TimeInForce = TimeInForce(2); // 如果不能全部成交则取消委托(全部成交或者全部撤销)

impl Default for TimeInForce {
    fn default() -> Self {
        Self(0)
    }
}

impl From<i32> for TimeInForce {
    fn from(other: i32) -> Self {
        Self(other)
    }
}

impl TimeInForce {
    pub fn is_nil(self) -> bool {
        self.0 == 0
    }
}

/** 交易类型 */
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct TradeClass(pub i32);

pub const TradeClass_Nil: TradeClass = TradeClass(0); // 空值
pub const TradeClass_SPOT: TradeClass = TradeClass(1); //  Spot Trading 现货交易
pub const TradeClass_FUTURE: TradeClass = TradeClass(2); // 指Future Trading 期货交易
pub const TradeClass_PERPETUAL: TradeClass = TradeClass(3); // 永续

impl Default for TradeClass {
    fn default() -> Self {
        Self(0)
    }
}

impl From<i32> for TradeClass {
    fn from(other: i32) -> Self {
        Self(other)
    }
}

impl TradeClass {
    pub fn is_nil(self) -> bool {
        self.0 == 0
    }
}

/** 合约标志 */
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
        const JUST_A_CONFIG     = 0x00001000; // VP分离交易对
        const SEPERATE_SYS_ACT_WLT  = 0x00002000; // 分离系统钱包帐号
        const ENABLE_UNP_FOR_MARGIN = 0x00004000; // 是否允许浮盈作为Margin
        const NO_STOP_LIQUIDATE     = 0x00008000; // 在仓位强平后，是否重新计算保证金，检查风险
        const NO_CHANGE_LEVERAGE    = 0x00010000; // NO_TWEAK_LEVERAGE
        const STOPLP_BY_SYS         = 0x00020000; // 是否强制成交止盈止损
        const FUNDING_BOTH_SIDE     = 0x00040000; // 双向资金费率
    }
}

/** 全仓杠杆保证金模式 */
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct MIRMode(pub i32);

pub const MIRMode_MIRM_DEFAULT: MIRMode = MIRMode(0); // 全仓杠杆模式 默认设定
pub const MIRMode_MIRM_MIR: MIRMode = MIRMode(1); // 用户设定MIR,系统约束MMR
pub const MIRMode_FROM_TO_FEECOIN: MIRMode = MIRMode(2); // 用户设定MIR, MMR = MIR/2

impl Default for MIRMode {
    fn default() -> Self {
        Self(0)
    }
}

impl From<i32> for MIRMode {
    fn from(other: i32) -> Self {
        Self(other)
    }
}

impl MIRMode {
    pub fn is_nil(self) -> bool {
        self.0 == 0
    }
}

/** 手续费计算方法 */
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct FeeMethod(pub i32);

pub const FeeMethod_FROM_TO: FeeMethod = FeeMethod(0); // 收入货币中支付。对于买卖双方，使用不同的货币支付手续费
pub const FeeMethod_FROM: FeeMethod = FeeMethod(1); // 使用购买行为中消费的币种为手续费
pub const FeeMethod_FROM_TO_FEECOIN: FeeMethod = FeeMethod(2); // 可以使用第三货币进行手续费抵扣。如果额度不足，则使用FROM_TO 的逻辑
pub const FeeMethod_FROM_FEECOIN: FeeMethod = FeeMethod(3); // 可以使用第三货币进行手续费抵扣。如果额度不足，则使用FROM    的逻辑

impl Default for FeeMethod {
    fn default() -> Self {
        Self(0)
    }
}

impl From<i32> for FeeMethod {
    fn from(other: i32) -> Self {
        Self(other)
    }
}

impl FeeMethod {
    pub fn is_nil(self) -> bool {
        self.0 == 0
    }
}

/** 钱包标志 */
bitflags! {
    #[derive(Default, Serialize, Deserialize)]
    pub struct WltFlags: u32 {
        const SKIP_TO_ADL       = 0x00000001; // 立即ADL如果爆仓
        const DENY_DEPO         = 0x00000002; // 禁止入金
        const DENY_WDRW         = 0x00000004; // 禁止出金
        const DENY_ORDER        = 0x00000008; // 禁止下单
    }
}

/** 钱包状态 */
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct WltStatus(pub i32);

pub const WltStatus_Nil: WltStatus = WltStatus(0); // 空
pub const WltStatus_NOT_ACTIVED: WltStatus = WltStatus(1); // 尚未激活
pub const WltStatus_NORMAL: WltStatus = WltStatus(2); // 正常状态
pub const WltStatus_LIQUIDATION: WltStatus = WltStatus(3); // 强平状态
pub const WltStatus_TAKEN_OVER: WltStatus = WltStatus(4); // 接管

impl Default for WltStatus {
    fn default() -> Self {
        Self(0)
    }
}

impl From<i32> for WltStatus {
    fn from(other: i32) -> Self {
        Self(other)
    }
}

impl WltStatus {
    pub fn is_nil(self) -> bool {
        self.0 == 0
    }
}

/** 交易对状态 */
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct MkStatus(pub i32);

pub const MkStatus_Nil: MkStatus = MkStatus(0); // 空值
pub const MkStatus_NORMAL: MkStatus = MkStatus(1); // 正常运行
pub const MkStatus_ADL: MkStatus = MkStatus(2); // 自动减仓
pub const MkStatus_PAUSE: MkStatus = MkStatus(3); // 暂停
pub const MkStatus_CLOSED: MkStatus = MkStatus(4); // 交易对已经关闭

impl Default for MkStatus {
    fn default() -> Self {
        Self(0)
    }
}

impl From<i32> for MkStatus {
    fn from(other: i32) -> Self {
        Self(other)
    }
}

impl MkStatus {
    pub fn is_nil(self) -> bool {
        self.0 == 0
    }
}

/** 钱包操作 */
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct WltOp(pub i32);

pub const WltOp_Nil: WltOp = WltOp(0); // 空值
pub const WltOp_DEPOSIT: WltOp = WltOp(1); // 存
pub const WltOp_WITHDRAW: WltOp = WltOp(2); // 取
pub const WltOp_PNL: WltOp = WltOp(3); // 已实现盈亏
pub const WltOp_SPOT: WltOp = WltOp(4); // 现货交易
pub const WltOp_TRAN_1_TO_MANY: WltOp = WltOp(5); // 一账户 与 多账户 进行操作
pub const WltOp_PNLISO: WltOp = WltOp(6); // 逐仓 已实现盈亏
pub const WltOp_GIFT: WltOp = WltOp(7); // 赠金
pub const WltOp_QUERY: WltOp = WltOp(8); // 查询

impl Default for WltOp {
    fn default() -> Self {
        Self(0)
    }
}

impl From<i32> for WltOp {
    fn from(other: i32) -> Self {
        Self(other)
    }
}

impl WltOp {
    pub fn is_nil(self) -> bool {
        self.0 == 0
    }
}

/** 仓位标志 */
bitflags! {
    #[derive(Default, Serialize, Deserialize)]
    pub struct PosFlags: u32 {
        const IS_MASTER         = 0x00000001; // 缺省仓位
        const ENABLE_STOPP      = 0x00000002; // 启用至盈价
        const ENABLE_STOPL      = 0x00000004; // 启用至损价
        const DISABLE_SHORT     = 0x00000008; // 禁止做空
        const DISABLE_LONG      = 0x00000010; // 禁止做多
    }
}

/* ------------------------------------------------------------ */
/* ------------------------------------------------------------ */
/* ------------------------------------------------------------ */

/** 委托单 */
#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Order {
    #[serde(skip_serializing_if = "is_default")] pub UId: String,        // 用户Id
    #[serde(skip_serializing_if = "is_default")] pub AId: String,        // 账户Id
    #[serde(skip_serializing_if = "is_default")] pub Sym: String,        // 交易对, 比如 BTC.USDT, ETH/USDT, BTC1912 等
    #[serde(skip_serializing_if = "is_default")] pub WId: String,        // 钱包ID
    #[serde(skip_serializing_if = "is_default")] pub OrdId: String,      // 服务器端为其分配的ID
    #[serde(skip_serializing_if = "is_default")] pub COrdId: String,     // 客户端为其分配的ID
    #[serde(skip_serializing_if = "is_default")] pub Dir: OrderDir,      // 委单方向 1=买/-1=卖,  0:Invalid, 1:BID/BUY, -1:ASK/SELL
    #[serde(skip_serializing_if = "is_default")] pub OType: OfferType,   // 报价类型
    #[serde(skip_serializing_if = "is_default")] pub Prz: Decimal,       // 价格
    #[serde(skip_serializing_if = "is_default")] pub Qty: Decimal,       // 数量
    #[serde(skip_serializing_if = "is_default")] pub QtyDsp: Decimal,    // 显示数量。如果为0,则显示全部Qty
    #[serde(skip_serializing_if = "is_default")] pub Tif: TimeInForce,   // 有效期
    #[serde(skip_serializing_if = "is_default")] pub OrdFlag: i32,       // 委托标志位,具体定义参考 OrderFlags;
    #[serde(skip_serializing_if = "is_default")] pub Via: OrderVia,      // 来源
    #[serde(skip_serializing_if = "is_default")] pub At: i64,            // 下单时间戳.单位:毫秒
    #[serde(skip_serializing_if = "is_default")] pub Upd: i64,           // 更新时间戳.单位:毫秒
    #[serde(skip_serializing_if = "is_default")] pub Until: i64,         // 有效期: 毫秒。绝对时间
    #[serde(skip_serializing_if = "is_default")] pub PrzChg: i32,        // 市价委托的最大档位(当撮合进行匹配的时候，会从Orderbook依档位进行)
    #[serde(skip_serializing_if = "is_default")] pub Frz: Decimal,       // 冻结金额
    #[serde(skip_serializing_if = "is_default")] pub ErrCode: i32,       // 错误代码
    #[serde(skip_serializing_if = "is_default")] pub ErrTxt: String,     // 错误文本
    #[serde(skip_serializing_if = "is_default")] pub Status: OrderStatus,// 状态
    #[serde(skip_serializing_if = "is_default")] pub QtyF: Decimal,      // 已成交 Qty Filled
    #[serde(skip_serializing_if = "is_default")] pub PrzF: Decimal,      // 已成交的平均价格 Prz Filled
    #[serde(skip_serializing_if = "is_default")] pub Val: Decimal,       // 合约价值, 对于PRZ_INVERSE的合约: - Dir * Qty / Prz; 对于正向合约 Dir * Qty * Prz
    #[serde(skip_serializing_if = "is_default")] pub PId: String,        // 仓位Id,如果指定了仓位Id,则本委托导致的的仓位变化，为修改指定的仓位
    /* ------------------------------------------------------------ */
    #[serde(skip_serializing_if = "is_default")] pub Lvr: f64,           // 只开仓模式: 杠杆设定
    #[serde(skip_serializing_if = "is_default")] pub StopL: f64,         // 只开仓模式: 止损价
    #[serde(skip_serializing_if = "is_default")] pub StopP: f64,         // 只开仓模式: 止盈价
    #[serde(skip_serializing_if = "is_default")] pub StopLPBy: StopBy,   // 只开仓模式: 止损止盈依据
    #[serde(skip_serializing_if = "is_default")] pub MIRMy: f64,         // 如果用户做全仓，就在这里设定值
    /* ------------------------------------------------------------ */
    #[serde(skip_serializing_if = "is_default")] pub StopBy: StopBy,     // 条件委托的判断依据
    #[serde(skip_serializing_if = "is_default")] pub StopPrz: Decimal,   // 条件委托的判断价格
    #[serde(skip_serializing_if = "is_default")] pub TraceRR: f64,       // 追踪委托中，回调的比率. Reverse Ratio. 小数。
    #[serde(skip_serializing_if = "is_default")] pub TraceMin: f64,      // 追踪的Min
    #[serde(skip_serializing_if = "is_default")] pub TraceMax: f64,      // 追踪的Max
    /* ------------------------------------------------------------ */
    #[serde(skip_serializing_if = "is_default")] pub TrgPrz: Decimal,    // 触发价格
    #[serde(skip_serializing_if = "is_default")] pub SzCls: f64,         // 平仓数量
    #[serde(skip_serializing_if = "is_default")] pub PnlCls: f64,        // 平仓收益
    #[serde(skip_serializing_if = "is_default")] pub PrzIO: f64,         // 仓位的最终开仓价格
    #[serde(skip_serializing_if = "is_default")] pub SzOpn: f64,         // 仓位的最终值
    #[serde(skip_serializing_if = "is_default")] pub Fee: f64,           // 已支付手续费
    /* ------------------------------------------------------------ */
    #[serde(skip_serializing_if = "is_default")] pub MM: f64,            // 委托保证金 Mgn Initial + 佣金
    #[serde(skip_serializing_if = "is_default")] pub FeeEst: f64,        // 预估的手续费: 按照手续费计算
    #[serde(skip_serializing_if = "is_default")] pub UPNLEst: f64,       // 预估的UPNL Predicatee
    #[serde(skip_serializing_if = "is_default")] pub VP: i64,            // 虚拟平台ID, 子交易所编号
}

impl Order {
    pub fn get_flags(&self) -> OrderFlags {
        unsafe {
            // 使用 from_bits_unchecked 而不是 from_bits_truncate 是因为服务器端随时可能定义一些属性，但这里不能失败
            OrderFlags::from_bits_unchecked(self.OrdFlag as u32)
        }
    }
    pub fn set_flags(&mut self, other: OrderFlags) {
        self.OrdFlag = other.bits() as i32;
    }
}

/** 持仓 */
#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Position {
    #[serde(skip_serializing_if = "is_default")] pub UId: String,        // 用户Id
    #[serde(skip_serializing_if = "is_default")] pub PId: String,        // 仓位Id
    #[serde(skip_serializing_if = "is_default")] pub AId: String,        // 账户Id
    #[serde(skip_serializing_if = "is_default")] pub Sym: String,        // 交易对, 比如 BTC.USDT, ETH/USDT, BTC1912 等
    #[serde(skip_serializing_if = "is_default")] pub WId: String,        // 钱包ID
    #[serde(skip_serializing_if = "is_default")] pub Sz: Decimal,        // 仓位(正数为多仓，负数为空仓)
    #[serde(skip_serializing_if = "is_default")] pub PrzIni: Decimal,    // 开仓平均价格
    #[serde(skip_serializing_if = "is_default")] pub RPNL: f64,          // 已实现盈亏
    #[serde(skip_serializing_if = "is_default")] pub Lever: f64,         // 杠杆
    #[serde(skip_serializing_if = "is_default")] pub MgnISO: Decimal,    // 逐仓下仓位保证金
    #[serde(skip_serializing_if = "is_default")] pub PNLISO: Decimal,    // 逐仓下已实现盈亏
    /** 下面是动态数据 */
    #[serde(skip_serializing_if = "is_default")] pub LeverMax: f64,      // 最大杠杆
    #[serde(skip_serializing_if = "is_default")] pub MMR: f64,           // 有效MMR
    #[serde(skip_serializing_if = "is_default")] pub MIR: f64,           // 有效MIR
    #[serde(skip_serializing_if = "is_default")] pub Flg: i32,           // 标志, 参考 PosFlags
    #[serde(skip_serializing_if = "is_default")] pub Val: f64,           // 计算值：价值,仓位现时的名义价值，受到标记价格价格的影响
    #[serde(skip_serializing_if = "is_default")] pub MMnF: f64,          // 保证金，被仓位使用并锁定的保证金
    #[serde(skip_serializing_if = "is_default")] pub MI: f64,            // 委托保证金 = 计算自已有委单 + 平仓佣金 + 开仓佣金 Mgn Initial
    #[serde(skip_serializing_if = "is_default")] pub UPNL: f64,          // 计算值：未实现盈亏 PNL==  Profit And Loss
    #[serde(skip_serializing_if = "is_default")] pub PrzLiq: f64,        // 计算值: 强平价格 亏光当前保证金的 (如果是多仓，并且标记价格低于PrzLiq,则会被强制平仓。/如果是空仓,并缺标记价格高于PrzLiq，则会被强制平仓
    #[serde(skip_serializing_if = "is_default")] pub PrzBr: f64,         // 计算值: 破产价格 BandRuptcy
    #[serde(skip_serializing_if = "is_default")] pub FeeEst: f64,        // 预估的平仓费
    /* ------------------------------------------------------------ */
    #[serde(skip_serializing_if = "is_default")] pub MIRMy: f64,         // 用户自定义杠杆
    /* ------------------------------------------------------------ */
    #[serde(skip_serializing_if = "is_default")] pub StopPBy: StopBy,    // 止盈方法
    #[serde(skip_serializing_if = "is_default")] pub StopP: f64,         // 止盈价
    #[serde(skip_serializing_if = "is_default")] pub StopLBy: StopBy,    // 止损方法
    #[serde(skip_serializing_if = "is_default")] pub StopL: f64,         // 止损价
    /* ------------------------------------------------------------ */
    #[serde(skip_serializing_if = "is_default")] pub ROE: f64,           // 净资产收益率
    #[serde(skip_serializing_if = "is_default")] pub ADLIdx: f64,        // ADLIdx, 这个是用来排序ADL的
    #[serde(skip_serializing_if = "is_default")] pub ADLLight: i32,      // ADL红绿灯
}

impl Position {
    pub fn get_flags(&self) -> PosFlags {
        unsafe {
            // 使用 from_bits_unchecked 而不是 from_bits_truncate 是因为服务器端随时可能定义一些属性，但这里不能失败
            PosFlags::from_bits_unchecked(self.Flg as u32)
        }
    }
    pub fn set_flags(&mut self, other: PosFlags) {
        self.Flg = other.bits() as i32;
    }
}

/** 钱包 */
#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Wlt {
    #[serde(skip_serializing_if = "is_default")] pub UId: String,        // 用户Id
    #[serde(skip_serializing_if = "is_default")] pub AId: String,        // 账户Id
    #[serde(skip_serializing_if = "is_default")] pub Coin: String,       // 货币类型
    #[serde(skip_serializing_if = "is_default")] pub WId: String,        // 钱包索引
    #[serde(skip_serializing_if = "is_default")] pub Depo: Decimal,      // 入金金额
    #[serde(skip_serializing_if = "is_default")] pub WDrw: Decimal,      // 出金金额
    #[serde(skip_serializing_if = "is_default")] pub PNL: Decimal,       // 已实现盈亏
    #[serde(skip_serializing_if = "is_default")] pub Frz: Decimal,       // 冻结金额
    /** 下面是统计值 */
    #[serde(skip_serializing_if = "is_default")] pub UPNL: f64,          // 未实现盈亏：根据持仓情况、标记价格 刷新
    #[serde(skip_serializing_if = "is_default")] pub MI: f64,            // 委托保证金 = 计算自已有委单 + 平仓佣金 + 开仓佣金 Mgn Initial
    #[serde(skip_serializing_if = "is_default")] pub MM: f64,            // 仓位保证金 + 平仓佣金 Mgn Maintaince
    #[serde(skip_serializing_if = "is_default")] pub RD: f64,            // 风险度 // Risk Degree.
    #[serde(skip_serializing_if = "is_default")] pub Wdrawable: f64,     // 可取余额 . 定时刷新
    #[serde(skip_serializing_if = "is_default")] pub Spot: Decimal,      // 现货交易出入金
    #[serde(skip_serializing_if = "is_default")] pub Gift: Decimal,      // 赠送金额 不允许取出
    #[serde(skip_serializing_if = "is_default")] pub PNLG: Decimal,      // Gift不为0的时候
    #[serde(skip_serializing_if = "is_default")] pub Status: WltStatus,  // 账户状态
    #[serde(skip_serializing_if = "is_default")] pub Flg: i32,           // WltFlags 钱包标志位
    #[serde(skip_serializing_if = "is_default")] pub VP: i64,            // 虚拟平台ID, 子交易所编号
}

impl Wlt {
    pub fn get_flags(&self) -> WltFlags {
        unsafe {
            // 使用 from_bits_unchecked 而不是 from_bits_truncate 是因为服务器端随时可能定义一些属性，但这里不能失败
            WltFlags::from_bits_unchecked(self.Flg as u32)
        }
    }
    pub fn set_flags(&mut self, other: WltFlags) {
        self.Flg = other.bits() as i32;
    }
}

/** 资金历史 */
#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct WltLog {
    #[serde(skip_serializing_if = "is_default")] pub UId: String,        // 用户Id
    #[serde(skip_serializing_if = "is_default")] pub AId: String,        // 账户Id
    #[serde(skip_serializing_if = "is_default")] pub Seq: String,        // 序列号
    #[serde(skip_serializing_if = "is_default")] pub Coin: String,       // 货币类型
    #[serde(skip_serializing_if = "is_default")] pub WId: String,        // 钱包Id
    #[serde(skip_serializing_if = "is_default")] pub Qty: Decimal,       // 数量
    #[serde(skip_serializing_if = "is_default")] pub Fee: Decimal,       // 手续费
    #[serde(skip_serializing_if = "is_default")] pub Peer: String,
    #[serde(skip_serializing_if = "is_default")] pub WalBal: Decimal,    // 余额
    #[serde(skip_serializing_if = "is_default")] pub At: i64,
    #[serde(skip_serializing_if = "is_default")] pub Op: WltOp,
    #[serde(skip_serializing_if = "is_default")] pub Via: OrderVia,
    #[serde(skip_serializing_if = "is_default")] pub Info: String,
    #[serde(skip_serializing_if = "is_default")] pub ErrCode: i32, // 参考 ErrorCode
    #[serde(skip_serializing_if = "is_default")] pub Stat: OrderStatus,
    #[serde(skip_serializing_if = "is_default")] pub VP: i64,            // 虚拟平台ID
}

/** 成交记录 */
#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct TrdRec {
    #[serde(skip_serializing_if = "is_default")] pub UId: String,        // 用户Id
    #[serde(skip_serializing_if = "is_default")] pub AId: String,        // 账户Id
    #[serde(skip_serializing_if = "is_default")] pub Sym: String,        // 交易对符号
    #[serde(skip_serializing_if = "is_default")] pub WId: String,        // 钱包Id
    #[serde(skip_serializing_if = "is_default")] pub MatchId: String,    // 撮合ID
    #[serde(skip_serializing_if = "is_default")] pub OrdId: String,      // 委托单ID
    #[serde(skip_serializing_if = "is_default")] pub Sz: Decimal,        // 数量
    #[serde(skip_serializing_if = "is_default")] pub Prz: Decimal,       // 价格
    #[serde(skip_serializing_if = "is_default")] pub Fee: Decimal,       // 手续费
    #[serde(skip_serializing_if = "is_default")] pub FeeCoin: String,    // 手续费币种
    #[serde(skip_serializing_if = "is_default")] pub At: i64,            // 时间戳,毫秒
    #[serde(skip_serializing_if = "is_default")] pub Via: OrderVia,      // 委托来源
    #[serde(skip_serializing_if = "is_default")] pub PAId: String,       // 对手账户Id
    #[serde(skip_serializing_if = "is_default")] pub Liq: f64,           // 强平价格
    #[serde(skip_serializing_if = "is_default")] pub Br: f64,            // 破产价格
    #[serde(skip_serializing_if = "is_default")] pub Lvr: f64,           // Lever
    #[serde(skip_serializing_if = "is_default")] pub PrzM: f64,          // 标记价格
    #[serde(skip_serializing_if = "is_default")] pub PId: String,        // 仓位Id
    #[serde(skip_serializing_if = "is_default")] pub GrossVal: f64,      // 本成交单的价值
    // pub HomeNotional: f64,
    // pub foreignNotional: f64,
    #[serde(skip_serializing_if = "is_default")] pub Gift: f64,          // 赠金
    /* ------------------------------------------------------------ */
    /** 下面的数据，来自Trdsum */
    #[serde(skip_serializing_if = "is_default")] pub BAvg: f64,          // 平均买入价
    #[serde(skip_serializing_if = "is_default")] pub NBid: f64,          // 计算平均值的买入量
    #[serde(skip_serializing_if = "is_default")] pub AAvg: f64,          // 平均卖出价
    #[serde(skip_serializing_if = "is_default")] pub NAsk: f64,          // 计算平均值的卖出量
    #[serde(skip_serializing_if = "is_default")] pub SzBid: f64,         // 统计周期内买入量
    #[serde(skip_serializing_if = "is_default")] pub SzAsk: f64,         // 统计周期内卖出量
    #[serde(skip_serializing_if = "is_default")] pub NumBid: i64,        // 统计周期内买入次数
    #[serde(skip_serializing_if = "is_default")] pub NumAsk: i64,        // 统计周期内卖出次数
    /* ------------------------------------------------------------ */
    #[serde(skip_serializing_if = "is_default")] pub MPL: i64,          // 算力等级
    #[serde(skip_serializing_if = "is_default")] pub MPB: f64,          // 买入算力 Mine Power for Bid
    #[serde(skip_serializing_if = "is_default")] pub MPA: f64,          // 卖出算力 Mine Power for Ask
    #[serde(skip_serializing_if = "is_default")] pub MPS: f64,          // 算力相关量. 可能并不会等于 Sz
    /* ------------------------------------------------------------ */
    #[serde(skip_serializing_if = "is_default")] pub Ext: String,       // 扩展字段
    /* ------------------------------------------------------------ */
    /** 下面字段用来描述开平仓和收益的 */
    #[serde(skip_serializing_if = "is_default")] pub PrzIC: f64,        // 平仓操作的开仓价
    #[serde(skip_serializing_if = "is_default")] pub SzCls: f64,        // 平仓数量
    #[serde(skip_serializing_if = "is_default")] pub PnlCls: f64,       // 平仓收益
    #[serde(skip_serializing_if = "is_default")] pub PrzIO: f64,        // 仓位的最终开仓价格
    #[serde(skip_serializing_if = "is_default")] pub SzOpn: f64,        // 仓位的最终值
    /* ------------------------------------------------------------ */
    #[serde(skip_serializing_if = "is_default")] pub StopL: f64,        // 止损
    #[serde(skip_serializing_if = "is_default")] pub StopP: f64,        // 止盈
    /* ------------------------------------------------------------ */
    #[serde(skip_serializing_if = "is_default")] pub VP: i64,           // 虚拟平台ID
}

/** 交易对 */
#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct AssetD {
    #[serde(skip_serializing_if = "is_default")] pub Sym: String,          // 交易对符号, BTC.USDT, ETH/ETH ...
    #[serde(skip_serializing_if = "is_default")] pub DspN: String,         // 交易对显示名字, 如: BTC永续
    #[serde(skip_serializing_if = "is_default")] pub Beg: i64,             // 开始时间
    #[serde(skip_serializing_if = "is_default")] pub Expire: i64,          // 到期日期 永续
    /* ------------------------------------------------------------ */
    #[serde(skip_serializing_if = "is_default")] pub PrzMaxChg: i32,       // 市价委托的撮合的最多次数。比如5
    #[serde(skip_serializing_if = "is_default")] pub PrzMinInc: Decimal,   // 最小的价格变化 0.5 USD
    #[serde(skip_serializing_if = "is_default")] pub PrzMax: Decimal,      // 最大委托价格 1,000,000
    #[serde(skip_serializing_if = "is_default")] pub OrderMaxQty: Decimal, // 最大委托数量 10,000,000
    #[serde(skip_serializing_if = "is_default")] pub LotSz: Decimal,       // 最小合约数量  这个就是每次买卖的合约数量必须是LotSz的倍数。
    /* 保证金计算相关参数 开始 */
    #[serde(skip_serializing_if = "is_default")] pub PrzM: Decimal,        // 标记价格 8103.14
    #[serde(skip_serializing_if = "is_default")] pub MIR: Decimal,         // 起始保证金 1.00% + 开仓佣金 + 平仓佣金 Mgn Initial Ratio
    #[serde(skip_serializing_if = "is_default")] pub MMR: Decimal,         // 维持保证金  0.50% + 平仓佣金 + 资金费率 Mgn Maintaince Ratio
    #[serde(skip_serializing_if = "is_default")] pub PrzMBiasR: f64,       // 当前价格的最大偏离率
    /* 保证金计算相关参数 结束 */
    /* 统计信息 */
    #[serde(skip_serializing_if = "is_default")] pub PrzLatest: Decimal,   // 最新成交价格
    #[serde(skip_serializing_if = "is_default")] pub DirLatest: OrderDir,  // 最新成交的方向
    // 最新的成交方向
    #[serde(skip_serializing_if = "is_default")] pub TotalVol: f64,        // 总交易量 30,585,913,058
    #[serde(skip_serializing_if = "is_default")] pub OpenInterest: f64,    // 持仓量  99,192,762
    #[serde(skip_serializing_if = "is_default")] pub Turnover: f64,        // 总成交额 26,293.1141 XBT
    #[serde(skip_serializing_if = "is_default")] pub PrzIndex: Decimal,    // 指数价格
    #[serde(skip_serializing_if = "is_default")] pub AssetSz: i64,         // 合约大小
    #[serde(skip_serializing_if = "is_default")] pub PosLmtStart: i64,     // 当总开仓到达这个数字，启动个人开仓率限制。
    #[serde(skip_serializing_if = "is_default")] pub PrzRFMin: f64,        // 当前涨跌价格范围 Prz Rise Fall Range
    #[serde(skip_serializing_if = "is_default")] pub PrzRFMax: f64,        // 当前涨跌价格范围最大值
    /* 佣金费率 */
    #[serde(skip_serializing_if = "is_default")] pub FeeMkrR: Decimal,     // 提供流动性的费率 FeeMkrR
    #[serde(skip_serializing_if = "is_default")] pub FeeTkrR: Decimal,     // 消耗流动性的费率
    #[serde(skip_serializing_if = "is_default")] pub Mult: Decimal,        // Order中，Qty必须是Mult的倍数
    #[serde(skip_serializing_if = "is_default")] pub FromC: String,        // 从什么货币 购买行为消耗的货币符号
    #[serde(skip_serializing_if = "is_default")] pub ToC: String,          // 兑换为 什么货币  购买行为得到的货币符号
    #[serde(skip_serializing_if = "is_default")] pub PrzCls: f64,          // 最近一个K线周期的收盘价。如果某K线指标被配置并指定了更新PrzCls,则此价格会得到更新
    #[serde(skip_serializing_if = "is_default")] pub MIRMd: MIRMode,       // 全仓杠杆保证金模式
    /* ------------------------------------------------------------ */
    #[serde(skip_serializing_if = "is_default")] pub TrdCls: TradeClass,   // 交易类型, 期货、现货
    #[serde(skip_serializing_if = "is_default")] pub MkSt: MkStatus,       // 市场状态
    #[serde(skip_serializing_if = "is_default")] pub Flag: i32,            // 交易对标志位, 具体定义参考 AssetFlags
    // 标志, 正向报价，反向报价
    #[serde(skip_serializing_if = "is_default")] pub SettleCoin: String,         // 结算货币
    #[serde(skip_serializing_if = "is_default")] pub QuoteCoin: String,          // 报价货币
    #[serde(skip_serializing_if = "is_default")] pub SettleR: Decimal,           // 结算费率
    #[serde(skip_serializing_if = "is_default")] pub DenyOpenAfter: i64,         // 时间节点：当越过了DenyOpenAfter后，不允许开新仓
    #[serde(skip_serializing_if = "is_default")] pub OrderMinQty: Decimal,       // 最小委托数量
    /** 永续合约专属数据 */
    #[serde(skip_serializing_if = "is_default")] pub InterestBaseSym: String,    // 基础货币利率符号
    #[serde(skip_serializing_if = "is_default")] pub InterestQuoteSym: String,   // 计价货币利率符号
    #[serde(skip_serializing_if = "is_default")] pub FundingPremiumSym: String,  // 资金费用溢价符号
    /** 资金费率 */
    #[serde(skip_serializing_if = "is_default")] pub FundingLongR: f64,          // 多仓资金费率
    #[serde(skip_serializing_if = "is_default")] pub FundingShortR: f64,         // 空仓资金费率
    #[serde(skip_serializing_if = "is_default")] pub FundingInterval: u32,       // 资金费用收取间隔 秒
    #[serde(skip_serializing_if = "is_default")] pub FundingNext: i64,           // 下一个资金费率结算的时间, 时间戳 毫秒
    #[serde(skip_serializing_if = "is_default")] pub FundingPredictedR: f64,     // 预测费率
    #[serde(skip_serializing_if = "is_default")] pub FundingOffset: i64,         // 每日0点后的 FundingOffset 毫秒后 为第一个结算时间点
    #[serde(skip_serializing_if = "is_default")] pub FundingTolerance: f64,      // 资金费率计算参数: 公差
    #[serde(skip_serializing_if = "is_default")] pub FundingFeeR: Decimal,       // Funding结算佣金
    // /* ------------------------------------------------------------ */
    // #[serde(skip_serializing_if = "is_default")] pub FundingAId: String,         // 资金结算佣金账户
    // #[serde(skip_serializing_if = "is_default")] pub InsurAIdLong: String,       // 多仓保险账户
    // #[serde(skip_serializing_if = "is_default")] pub ServeAId: String,           // 服务费账户
    // #[serde(skip_serializing_if = "is_default")] pub InsurAId: String,           // 空仓保险账户
    #[serde(skip_serializing_if = "is_default")] pub Grp: i64,                   // 分组,当前主要前端UI显示是用来隐藏或分类用;
}

impl AssetD {
    pub fn get_flags(&self) -> AssetFlags {
        unsafe {
            // 使用 from_bits_unchecked 而不是 from_bits_truncate 是因为服务器端随时可能定义一些属性，但这里不能失败
            AssetFlags::from_bits_unchecked(self.Flag as u32)
        }
    }
    pub fn set_flags(&mut self, other: AssetFlags) {
        self.Flag = other.bits() as i32;
    }
}

/** 交易对扩展属性 */
#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct V2AssetCfg {
    #[serde(skip_serializing_if = "is_default")] pub Sym: String,        // 符号, BTC.USDT, ETH/ETH ...
    #[serde(skip_serializing_if = "is_default")] pub FM: FeeMethod,      // 手续费计费方法
    #[serde(skip_serializing_if = "is_default")] pub FeeCoin: String,    // 手续费币种
    #[serde(skip_serializing_if = "is_default")] pub FeeDiscR: Decimal,  // 折扣率
    #[serde(skip_serializing_if = "is_default")] pub OnAt: u64,          // 开放交易时间 (日内,毫秒)
    #[serde(skip_serializing_if = "is_default")] pub OffAt: u64,         // 关闭交易时间 (日内,毫秒)
    #[serde(skip_serializing_if = "is_default")] pub RiseR: i64,         // 价格涨价幅度 万分比 * 10000
    #[serde(skip_serializing_if = "is_default")] pub FallR: i64,         // 价格跌价幅度 万分比 * 10000
    #[serde(skip_serializing_if = "is_default")] pub PrzMin: f64,        // 最小价格
    #[serde(skip_serializing_if = "is_default")] pub LmtBid: f64,        // 买入量
    #[serde(skip_serializing_if = "is_default")] pub LmtAsk: f64,        // 卖出量
    #[serde(skip_serializing_if = "is_default")] pub LmtBidAsk: f64,     // 买入卖出总量
    #[serde(skip_serializing_if = "is_default")] pub BidPrzR: f64,       // 委托的买价偏离盘口比例(小数)
    #[serde(skip_serializing_if = "is_default")] pub AskPrzR: f64,       // 委托的买价偏离盘口比例(小数)
    #[serde(skip_serializing_if = "is_default")] pub LmtNetAsk: f64,     // 每统计周期 净卖量。如果为0，则表示不进行检查
    #[serde(skip_serializing_if = "is_default")] pub SumAt: u64,         // 从0点开始，在每天的什么时间，开始重置统计值(绝对时间,毫秒)
    #[serde(skip_serializing_if = "is_default")] pub SumInterval: u64,   // 重置间隔
    #[serde(skip_serializing_if = "is_default")] pub SumResetNext: u64,  // 下次重置
    #[serde(skip_serializing_if = "is_default")] pub SzForAvg: f64,      // 求用户的最近的买入价格的量
    #[serde(skip_serializing_if = "is_default")] pub FeeMkrMin: Decimal, // Maker最低手续费
    #[serde(skip_serializing_if = "is_default")] pub FeeTkrMin: Decimal, // Taker最低手续费
    /** 下面是挖矿相关设定 */
    #[serde(skip_serializing_if = "is_default")] pub SzMaxFM: f64,       // 每日有挖矿算力的交易量
    #[serde(skip_serializing_if = "is_default")] pub NumMaxFM: f64,      // 每日有挖矿算力的交易次数
    #[serde(skip_serializing_if = "is_default")] pub ExpRatio: f64,      // 涨经验的交易量完成率.当交易量达到 SzMaxFM * ExpRatio Exp ++
    #[serde(skip_serializing_if = "is_default")] pub ExpMax: i64,        // 最大Exp
    #[serde(skip_serializing_if = "is_default")] pub Flag: i32,          // AssetFlags 标志位
    // /** 一些通用参数 */
    // #[serde(skip_serializing_if = "is_default")] pub F0: f64,
    // #[serde(skip_serializing_if = "is_default")] pub F1: f64,
    // #[serde(skip_serializing_if = "is_default")] pub F2: f64,
    // #[serde(skip_serializing_if = "is_default")] pub F3: f64,
    // #[serde(skip_serializing_if = "is_default")] pub F4: f64,
    // #[serde(skip_serializing_if = "is_default")] pub F5: f64,
    // #[serde(skip_serializing_if = "is_default")] pub F6: f64,
    // #[serde(skip_serializing_if = "is_default")] pub F7: f64,
    // #[serde(skip_serializing_if = "is_default")] pub F8: f64,
    // #[serde(skip_serializing_if = "is_default")] pub F9: f64,
    // #[serde(skip_serializing_if = "is_default")] pub I0: i64,
    // #[serde(skip_serializing_if = "is_default")] pub I1: i64,
    // #[serde(skip_serializing_if = "is_default")] pub I2: i64,
    // #[serde(skip_serializing_if = "is_default")] pub I3: i64,
}

impl V2AssetCfg {
    pub fn get_flags(&self) -> AssetFlags {
        unsafe {
            // 使用 from_bits_unchecked 而不是 from_bits_truncate 是因为服务器端随时可能定义一些属性，但这里不能失败
            AssetFlags::from_bits_unchecked(self.Flag as u32)
        }
    }
    pub fn set_flags(&mut self, other: AssetFlags) {
        self.Flag = other.bits() as i32;
    }
}

/** 风险限额定义 */
#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct RiskLimitDef {
    #[serde(skip_serializing_if = "is_default")] pub Name: String,       // 本配置的名称
    #[serde(skip_serializing_if = "is_default")] pub Sym: String,        // Symbol 交易对。或特定的名字，比如 XBTUSD_01,XBTUSD_99
    #[serde(skip_serializing_if = "is_default")] pub Base: f64,          // Base Risk Limit 当 Pos       Val < Base 的时候
    #[serde(skip_serializing_if = "is_default")] pub BaseMMR: f64,       // Base Maintenance Margin      Val < Base 的时候 MMR
    #[serde(skip_serializing_if = "is_default")] pub BaseMIR: f64,       // Initial Margin               Val < Base 的时候 MIR
    #[serde(skip_serializing_if = "is_default")] pub Step: f64,          // Step                         StepS = math.Ceil((Val - Base)/Step) 表示递增次数
    #[serde(skip_serializing_if = "is_default")] pub StepMR: f64,        // StepM						每次递增的时候，MMR MIR 的增量
    #[serde(skip_serializing_if = "is_default")] pub PosSzMax: f64,      // 最大持仓
    #[serde(skip_serializing_if = "is_default")] pub StepIR: f64,        // StepIR						每次递增的时候，MIR 的增量
    #[serde(skip_serializing_if = "is_default")] pub MaxOrdVal: f64,     // 单笔委托的最大价值
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
    #[serde(skip_serializing_if = "is_default")] pub wid: String,          // 主键：资金账户id， uid+Wtype
    #[serde(skip_serializing_if = "is_default")] pub uid: String,          // 用户账号uid
    #[serde(skip_serializing_if = "is_default")] pub coin: String,         // 币种名称（如BTC/ETH等）
    #[serde(skip_serializing_if = "is_default")] pub mainBal: Decimal,     // 主账户余额
    #[serde(skip_serializing_if = "is_default")] pub mainLock: Decimal,    // 主账户锁币额度
    #[serde(skip_serializing_if = "is_default")] pub otcBal: Decimal,      // otc法币账户余额
    #[serde(skip_serializing_if = "is_default")] pub otcLock: Decimal,     // otc锁币额度
    #[serde(skip_serializing_if = "is_default")] pub financeBal: Decimal,  // 理财额度
    #[serde(skip_serializing_if = "is_default")] pub pawnBal: Decimal,     // 质押额度
    #[serde(skip_serializing_if = "is_default")] pub creditNum: Decimal,   // 欠贷款额度【负】
    #[serde(skip_serializing_if = "is_default")] pub wdLimit: Decimal,     // 提现限额
    #[serde(skip_serializing_if = "is_default")] pub depositLock: Decimal, // 充值锁定（交易挖矿）
    #[serde(skip_serializing_if = "is_default")] pub cTime: i64,           // 账户创建时间（秒）
    #[serde(skip_serializing_if = "is_default")] pub updTime: i64,         // 账户创建时间（秒），每次更改刷新
    #[serde(skip_serializing_if = "is_default")] pub flag: i64,            // 账户标记（备用）
    #[serde(skip_serializing_if = "is_default")] pub memo: String,         // 账户备注
    #[serde(skip_serializing_if = "is_default")] pub email: String,        // 账户名email
}

/** 通过用户中心查询资产时，返回的撮合的资产的数据结构, 本质上和 Wlt 的信息一致. */
#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CcsMatcherWallet {
    #[serde(skip_serializing_if = "is_default")] pub wType: String,    // 币种
    #[serde(skip_serializing_if = "is_default")] pub Num: Decimal,     // 金额（入金总额-出金总额）
    #[serde(skip_serializing_if = "is_default")] pub PNL: Decimal,     // 已实现盈亏
    #[serde(skip_serializing_if = "is_default")] pub Frz: Decimal,     // 冻结金额
    #[serde(skip_serializing_if = "is_default")] pub UPNL: f64,        // 未实现盈亏：根据持仓情况、标记价格 刷新，统计值
    #[serde(skip_serializing_if = "is_default")] pub PNLISO: f64,      // 逐仓下已实现盈亏
    #[serde(skip_serializing_if = "is_default")] pub MI: f64,          // 委托保证金 = 计算自已有委单 + 平仓佣金 + 开仓佣金 Mgn Initial
    #[serde(skip_serializing_if = "is_default")] pub MM: f64,          // 仓位保证金 + 平仓佣金 Mgn Maintaince
    #[serde(skip_serializing_if = "is_default")] pub RD: f64,          // 风险度 // Risk Degree.
    #[serde(skip_serializing_if = "is_default")] pub balance: f64,     // 计算得出的余额，仅当时有效
    #[serde(skip_serializing_if = "is_default")] pub wdrawable: f64,   // 撮合计算出来的可取余额
    #[serde(skip_serializing_if = "is_default")] pub Gift: Decimal,    // 合约赠金
    #[serde(skip_serializing_if = "is_default")] pub PNLG: Decimal,    // 合约赠金盈亏
}

/* ------------------------------------------------------------ */
/* ------------------------------------------------------------ */
/* ------------------------------------------------------------ */

/** 错误代码定义(撮合引擎部分) */
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ErrorCode {
    NOERROR = 0,                   // 没有错误
    GENERAL = 1,                   // 数据错误
    DATA = 2,                      // 数据错误
    NOT_IMPLEMENTED = 3,           // 服务器未实现
    NO_MARGIN = 4,                 // 保证金不足
    FATAL = 5,                     // 致命错误
    NOT_FOUND = 6,                 // 未找到
    UNKNOWN_DIR = 7,               // 未知的委托方向
    INVALID_CODE = 8,              // 操作码错误
    EXISTS = 9,                    // 已存在
    NOT_FOUND_ORD = 10,            // 未找到委托
    PRZ_INVALID = 11,              // 价格错误
    EXPIRED = 12,                  // 已过期
    NOT_SUFFICIENT = 13,           // 资金不足
    WILLFILL = 14,                 // 对于PostOnly，本委托会成交
    EXECUTE_FAIL = 15,             // 对FillOrKill委托，这表示执行撮合失败
    EXCEED_LIMIT_MINVAL = 16,      // 超过限制
    ORDQTY_TOO_BIG_TOO_SMALL = 17, // 委托价值太小
    EXCEED_LIMIT_PRZ_QTY = 18,     // 价格或者数量超出限制
    DENYOPEN_BY_POS = 19,          // 仓位超出限制
    DENYOPEN_BY_RD = 20,           // 禁止开仓
    TRADE_STOPED = 21,             // 交易暂停
    EXCEED_PRZ_LIQ = 22,           // 超过强平价格
    TOO_MANY_ORDER = 23,           // 太多的委托
    DENYOPEN_BY_TIME = 24,         // 超出开仓时间限制
    MD5_INVALID = 25,              // MD5签名验证错误
    RATELIMIT = 26,                // 限速
    USER_CANCELED = 27,            // 用户撤销
    NOT_FOUND_WLT = 28,            // 无法找到钱包
    NOT_FOUND_MKT = 29,            // 未找到交易对
    EXCEED_MAXORDVAL = 30,         // 超过最大委托价值
    WILL_LIQUIDATE = 31,           // 将导致爆仓、强平
    NOT_IN_TRADE_PERIOD = 32,      // 非交易时间
    EXCEED_RAISE_FALL_R = 33,      // 超过涨跌停价格闲置
    PRZ_TOO_LOW = 34,              // 超出最小价格闲置
    EXCEED_TRADE_VOL = 35,         // 超出交易量限制
    EXCEED_TRADE_COUNT = 36,       // 超出交易次数闲置
    EXCEED_ASK_BID_PRZ_RATE = 37,  // 委托价格 超过盘口最新价格偏离
    EXCEED_TRDSUM = 39,            // TRDSUM限制
    OVERLOAD = 40,                 // OVERLOAD
    TOO_MANY_POS = 41,             // TOO_MANY_POS
    CHANNEL_BUSY = 42,             // CHANNEL_BUSY
    CANT_ADD_MGN = 43,             // CANT_ADD_MGN
    WILLCLOSE = 44,                // 将导致平仓
    REDUCE_ONLY = 45,              // 只减仓委托
    AUTO_CANCEL = 46,              // 自动撤单
    NO_DEFAULT_RISKLIMIT = 64,     // 没有指定风险限额
    NOT_POSITION_ISO = 65,         // 不是一个逐仓
    TIMEOUT = 99,                  // 执行超时
}

impl Default for ErrorCode {
    fn default() -> ErrorCode {
        ErrorCode::NOERROR
    }
}
