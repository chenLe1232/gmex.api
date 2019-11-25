// GMEX-API 数据结构定义
//

pub const GMEX_API_VERSION: &str = "1.0.0";

pub const GMEX_HTTP_URL_MARKET: &str = "https://api-market.gmex.io/v1/rest";
pub const GMEX_HTTP_URL_TRADE: &str = "https://api-trade.gmex.io/v1/rest";

// wss://s0.gmex.io/v1/market wss://s0.gmex.io/v1/trade
pub const GMEX_WS_URL_MARKET: &str = "wss://api-market.gmex.io/v1/market";
pub const GMEX_WS_URL_TRADE: &str = "wss://api-trade.gmex.io/v1/trade";

#[macro_use]
extern crate bitflags;

pub use rust_decimal::Decimal;
pub use rust_decimal_macros::dec;

mod types;
mod mkt;
mod msg;

pub use crate::types::{
    ErrorCode,
    OrderDir,
    OrderVia,
    OrderStatus,
    OfferType,
    OrdFlags,
    StopBy,
    TimeInForce,
    TradeClass,
    AssetFlags,
    FeeMethod,
    WltStatus,
    MkStatus,
    WltOp,
    Ord,
    Position,
    Wlt,
    WltLog,
    TrdRec,
    AssetD,
    V2AssetCfg,
    RiskLimitDef,
    CcsMainWallet,
    CcsMatcherWallet,
};
pub use crate::mkt::{
    MktCompositeIndexTick,
    MktInstrumentTick,
    MktOrderItem,
    MktTradeItem,
    MktKLineType,
    MktKLineItem,
    MktOrder20Result,
    MktQueryKLineHistoryRequestArgs,
    MktQueryKLineHistoryResult,
};

pub use crate::msg::{
    HttpTradeRequestMessage,
    HttpResponseMessage,
    WsResponseMessage,
};