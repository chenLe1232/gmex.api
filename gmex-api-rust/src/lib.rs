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

mod helper;
mod mkt;
mod msg;
mod types;

pub use crate::helper::get_now_msec;
pub use crate::mkt::{
    MktCompositeIndexTick, MktInstrumentTick, MktKLineItem, MktKLineType, MktOrder20Result, MktOrderItem, MktQueryKLineHistoryRequestArgs,
    MktQueryKLineHistoryResult, MktTradeItem,
};
pub use crate::msg::{HttpResponseMessage, HttpTradeRequestMessage, WsResponseMessage};
pub use crate::types::{
    AssetD, AssetFlags, CcsMainWallet, CcsMatcherWallet, ErrorCode, FeeMethod, MkStatus, OfferType, Ord, OrdFlags, OrderDir, OrderStatus, OrderVia, Position,
    RiskLimitDef, StopBy, TimeInForce, TradeClass, TrdRec, V2AssetCfg, Wlt, WltLog, WltOp, WltStatus,
};
