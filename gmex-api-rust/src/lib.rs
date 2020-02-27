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
pub mod types;

pub use crate::helper::{is_default, time_now_msec, time_now_sec};

pub use crate::mkt::{
    MktCompositeIndexTick, MktInstrumentTick, MktKLineItem, MktKLineType, MktOrder20Result, MktOrderItem,
    MktQueryKLineHistoryRequestArgs, MktQueryKLineHistoryResult, MktTradeItem,
};
pub use crate::msg::{HttpResponseMessage, HttpTradeRequestMessage, WsResponseMessage};
pub use crate::types::{
    AssetD, AssetFlags, CcsMainWallet, CcsMatcherWallet, ErrorCode, FeeMethod, MkStatus, OfferType, Order, OrderDir,
    OrderFlags, OrderStatus, OrderVia, Position, RiskLimitDef, StopBy, TimeInForce, TradeClass, TrdRec, V2AssetCfg,
    Wlt, WltLog, WltOp, WltStatus,
};

//
// cargo test -- --nocapture
//
#[cfg(test)]
mod tests {
    use super::*;

    // #[macro_use]
    // extern crate serde_json;
    // use serde::{Deserialize, Serialize};
    use serde_json;

    #[test]
    fn test_assert_flags_work() {
        // 0x20000000 536870912 + 3(PRZ_INVERSE, DO_ADL),  AUTO_SETTLE=4
        let data = r#"{
            "Sym": "BTC.BTC", "DirLatest":5,
            "Flag": 536870915
        }"#;
        let mut v: AssetD = serde_json::from_str(data).unwrap();
        println!("XXX:json1={}", serde_json::to_string(&v).unwrap());

        let mut flags = v.get_flags();
        println!("XXX:flags={}", flags.bits());
        assert_eq!(flags.bits(), 536870915);
        assert!(flags.contains(AssetFlags::PRZ_INVERSE | AssetFlags::DO_ADL));
        assert!(!flags.contains(AssetFlags::AUTO_SETTLE));
        flags.insert(AssetFlags::AUTO_SETTLE);
        assert!(flags.contains(AssetFlags::AUTO_SETTLE));
        // let a:types::OrderDir_e = v.DirLatest.into();
        // println!("XXX:a={:?}", a);

        v.DirLatest = types::OrderDir_BID;
        assert!(v.DirLatest.is_bid());
        println!("XXX:json2={}", serde_json::to_string(&v).unwrap());
    }
}
