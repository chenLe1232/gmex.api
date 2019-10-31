# GMEX WebSocket API (v1)

## 说明

目前 GMEX (https://www.gmex.io) 对于外提供 WebSocket API 开发接口， 供开发者获取行情数据和进行交易操作。
*请注意行情和交易是两个不同服务器*，行情接口无需认证可以自由访问，交易部分则需要用户开通 API-KEY 后通过自己的 KEY 认证授权后方可使用。

GMEX官方的生产环境：

```txt
官方网址： https://www.gmex.io
行情服务： wss://api-market.gmex.io/v1/market
交易服务： wss://api-trade.gmex.io/v1/trade
```


## 行情API

1. 获取交易对/合约列表： GetAssetD/GetAssetEx/GetCompositeIndex
```JavaScript
// 发送请求消息
{"req":"GetAssetD","rid":"0","expires":1537706670830}

// 收到返回消息
{
    "rid":"0",
    "code":0,
    "data":[
        {
            "Sym":"ETH1812",                    // 交易对名称
            "Beg":1,
            "Expire":1545984000000,
            "PrzMaxChg":1000,
            "PrzMinInc":0.05,
            "PrzMax":1000000,
            "OrderMaxQty":10000000,
            "OrderMinQty":1,
            "LotSz":1,
            "PrzM":244.8799999999999954525264911353588104248046875,
            "MIR":0.07,
            "MMR":0.05,
            "PrzLatest":244.95,
            "OpenInterest":2181200,
            "PrzIndex":244.8863,
            "PosLmtStart":10000000,
            "FeeMkrR":-0.0003,
            "FeeTkrR":0.0007,
            "Mult":1,
            "FromC":"ETH",
            "ToC":"USD",
            "TrdCls":2,
            "MkSt":1,
            "Flag":1,
            "SettleCoin":"ETH",
            "QuoteCoin":"ETH",
            "SettleR":0.0005,
            "DenyOpenAfter":1545980400000,
            "FundingLongR":0,             // 当前周期内的资金费率
            "FundingPredictedR":0,        // 下个周期预测的资金费率
            "FundingShortR":0,            // 当前未使用字段
            "FundingInterval": 55,        // 结算间隔(毫秒)
            "FundingNext": 56,            // 下次结算时间戳
            "FundingTolerance": 59,       // 偏移宽容度
            "FundingFeeR": 60             // Funding结算佣金
        },
        {"Sym":"BTC1812","Beg":1,"Expire":1545984000000,"PrzMaxChg":1000,"PrzMinInc":0.5,"PrzMax":1000000,"OrderMaxQty":10000000,"LotSz":1,"PrzM":6731.3100000000004001776687800884246826171875,"MIR":0.07,"MMR":0.05,"PrzLatest":6731.0,"OpenInterest":3431840,"PrzIndex":6737.3525,"PosLmtStart":10000000,"FeeMkrR":-0.0003,"FeeTkrR":0.0007,"Mult":1,"FromC":"BTC","ToC":"USD","TrdCls":2,"MkSt":1,"Flag":1,"SettleCoin":"BTC","QuoteCoin":"BTC","SettleR":0.0005,"DenyOpenAfter":1545980400000},
        {"Sym":"ETH1809","Beg":1,"Expire":1538121600000,"PrzMaxChg":1000,"PrzMinInc":0.05,"PrzMax":1000000,"OrderMaxQty":10000000,"LotSz":1,"PrzM":244.19999999999998863131622783839702606201171875,"MIR":0.07,"MMR":0.05,"PrzLatest":244.20,"OpenInterest":4500733,"PrzIndex":244.8863,"PosLmtStart":10000000,"FeeMkrR":-0.0003,"FeeTkrR":0.0007,"Mult":1,"FromC":"ETH","ToC":"USD","TrdCls":2,"MkSt":1,"Flag":1,"SettleCoin":"ETH","QuoteCoin":"ETH","SettleR":0.0005,"DenyOpenAfter":1538118000000},
        {"Sym":"BTC1809","Beg":1,"Expire":1538121600000,"PrzMaxChg":1000,"PrzMinInc":0.5,"PrzMax":1000000,"OrderMaxQty":10000000,"LotSz":1,"PrzM":6727.5500000000001818989403545856475830078125,"MIR":0.07,"MMR":0.05,"PrzLatest":6728.0,"OpenInterest":1451134,"PrzIndex":6737.3525,"PosLmtStart":10000000,"FeeMkrR":-0.0003,"FeeTkrR":0.0007,"Mult":1,"FromC":"BTC","ToC":"USD","TrdCls":2,"MkSt":1,"Flag":1,"SettleCoin":"BTC","QuoteCoin":"BTC","SettleR":0.0005,"DenyOpenAfter":1538118000000}
    ]
}
```

用户发送和接收到的所有消息统一采用JSON格式，发送请求的消息参数说明：

|参数 | 描述|
| :-----   | :-----   |
|req|用户的请求操作动作，如： GetAssetD，GetCompositeIndex，GetHistKLine等|
|rid|用户发送请求的唯一编号，由于websocket是异步通讯，用户需要通过匹配收到消息的rid和自己发送的rid来匹配操作和应答。|
|expires|消息超时，毫秒，建议每次发送请求时填写当前时间加1秒。一般宜在初始化时先用Time消息获取服务端时间,可以相对时差与服务端保持同步。|

有些交易对规则特别复杂，为此特别设置了一些扩展参数数据，对应的API指令为： GetAssetEx，使用和上面API一样。
注意返回的结果是数组，只有配置了的交易对才会有，没配置的则没有数据，以交易对Sym为主键。

交易对相关对应的结构定义如下：

```golang

//
// 合约标志
type AssetFlag int32

const (
    // 占位、无任何标志
    AssetFlag_CF_INVALID AssetFlag = 0
    // 例:盈亏 = 合约数量 * 乘数 * ( ( - 1/平仓价格) - ( - 1/开仓价格 ) ).如果是正向合约: 盈亏 = 合约数量 * 乘数 * ( ( 1 * 平仓价格 ) - ( 1 * 开仓价格 ) )
    AssetFlag_PRZ_INVERSE AssetFlag = 1
    // 在必要的情况下，进行自动减仓操作。未使用
    AssetFlag_DO_ADL AssetFlag = 2
    // 自动结算
    AssetFlag_AUTO_SETTLE AssetFlag = 4
    // 禁止开仓
    AssetFlag_DENY_OPEN AssetFlag = 8
    // 停止交易
    AssetFlag_TRADE_STOPPED AssetFlag = 16
    // 手续费率设定方法
    AssetFlag_FEE_R_FOR_BUYSELL AssetFlag = 32
    // 激活挖矿系统
    AssetFlag_ENABLE_MINING AssetFlag = 64
    // KNodelist更新时，更新价格区段
    AssetFlag_UPDATE_PRZ_LIMIT AssetFlag = 256
    // 数据失效
    AssetFlag_DATA_INVALID AssetFlag = 512
)

// **交易对/合约的结构定义**
type AssetD struct {
    Sym                 string  // 合约符合/交易对符号
    Beg                 int64   // 开始时间,毫秒
    Expire              int64   // 到期时间,毫秒
    PrzMaxChg           int32   // 市价委托的撮合的最多次数。比如5
    PrzMinInc           float64 // 最小的价格变化
    PrzMax              float64 // 最大委托价格
    OrderMaxQty         float64 // 最大委托数量
    OrderMinQty         float64 // 最小委托数量
    LotSz               float64 // 最小合约数量,每次买卖的合约数量必须是LotSz的倍数,当前只支持为1;
    PrzM                float64 // 标记价格
    MIR                 float64 // 起始保证金率
    MMR                 float64 // 维持保证金率
    PrzLatest           float64 // 最新成交价格
    TotalVol            float64 // 总交易量
    OpenInterest        int64   // 持仓量
    Turnover            float64 // 总成交额
    PrzIndex            float64 // 指数价格
    PosLmtStart         int64   // 个人持仓比例激活条件
    PrzRFMin            float64 // 当前涨跌价格范围 Prz Rise Fall Range
    PrzRFMax            float64 // 当前涨跌价格范围最大值
    FeeMkrR             float64 // 提供流动性的费率
    FeeTkrR             float64 // 消耗流动性的费率
    Mult                float64 // 乘数
    FromC               string  // 从什么货币
    ToC                 string  // 兑换为什么货币
    TrdCls              int32   // 交易类型, 1-现货交易, 2-期货交易, 3-永续
    MkSt                int32   // 合约、交易对的状态: 1-正常运行, 2-自动减仓, 3-暂停, 4-交易对已经关闭
    Flag                AssetFlag   // 合约标志, 位操作
    SettleCoin          string  // 结算货币
    QuoteCoin           string  // 报价货币
    SettleR             float64 // 结算费率
    DenyOpenAfter       int64   // 到期前禁止开仓时间,毫秒
    FundingLongR        float64 // 当前周期内的资金费率
    FundingPredictedR   float64 // 下个周期预测的资金费率
    FundingShortR       float64 // 当前未使用字段
    FundingInterval     uint32  // 结算间隔(毫秒)
    FundingNext         int64   // 下次结算时间戳
    FundingTolerance    float64 // 偏移宽容度
    FundingFeeR         float64 // Funding结算佣金
    FeeCoin             string  // 如果允许使用第三种货币支付手续费，则配置本项目
    FeeDiscR            float64 // 如果允许使用第三种货币支付手续费，这里配置折扣率
    Grp                 int64   // 交易对所属的分组ID，仅仅是一个逻辑分组概念.
}

// **定义手续分的收取方式**
type FeeMethod int32

const (
    FeeMethod_FM_IN_FROM_TO         FeeMethod = 0	// 收入货币中支付。对于买卖双方，使用不同的货币支付手续费
    FeeMethod_FM_IN_FROM            FeeMethod = 1	// 使用购买行为中消费的币种为手续费
    FeeMethod_FM_IN_FROM_TO_FEECOIN FeeMethod = 2	// 可以使用第三货币进行手续费抵扣。如果额度不足，则使用FROM_TO 的逻辑
    FeeMethod_FM_IN_FROM_FEECOIN    FeeMethod = 3	// 可以使用第三货币进行手续费抵扣。如果额度不足，则使用FROM    的逻辑
)

// **交易对的扩展配置数据**
type V2AssetCfg struct {
    // 交易对(合约对)
    Sym string `json:"Sym,omitempty"`
    // //////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // 手续费计费方法
    FM FeeMethod `json:"FM,omitempty"`
    // 尚未支持
    // 手续费，货币符号，如果未指定，则现货：按照收入额进行收取。期货：按照SettleCoin进行。
    // 如果指定了FeeCoin则从该币种钱包内进行扣除。注意到，如果该钱包余额不足，则依旧使用SettleCoin进行
    FeeCoin string `json:"FeeCoin,omitempty"`
    // 折扣率
    FeeDiscR gaea_Num.Flt `json:"FeeDiscR"`
    // 开放交易时间 (日内,毫秒)
    OnAt uint64 `json:"OnAt,omitempty"`
    // 关闭交易时间 (日内,毫秒)
    OffAt uint64 `json:"OffAt,omitempty"`
    // 价格涨价幅度 万分比 * 10000
    RiseR int64 `json:"RiseR,omitempty"`
    // 价格跌价幅度 万分比 * 10000
    FallR int64 `json:"FallR,omitempty"`
    // 最小价格
    PrzMin float64 `json:"PrzMin,omitempty"`
    // 买入量
    LmtBid float64 `json:"LmtBid,omitempty"`
    // 卖出量
    LmtAsk float64 `json:"LmtAsk,omitempty"`
    // 买入卖出总量
    LmtBidAsk float64 `json:"LmtBidAsk,omitempty"`
    // 买入次数
    LmtNumBid uint64 `json:"LmtNumBid,omitempty"`
    // 卖出次数
    LmtNumAsk uint64 `json:"LmtNumAsk,omitempty"`
    // 买入卖出总次数
    LmtNumBidAsk uint64 `json:"LmtNumBidAsk,omitempty"`
    // 委托的买价偏离盘口比例(小数)
    BidPrzR float64 `json:"BidPrzR,omitempty"`
    // 委托的买价偏离盘口比例(小数)
    AskPrzR float64 `json:"AskPrzR,omitempty"`
    // 每统计周期 净卖量。如果为0，则表示不进行检查
    LmtNetAsk float64 `json:"LmtNetAsk,omitempty"`
    // 每统计周期 卖/买比率. 如果为0，则表示不进行检查
    LmtAskQBid float64 `json:"LmtAskQBid,omitempty"`
    // 从0点开始，在每天的什么时间，开始重置统计值(绝对时间,毫秒)
    SumAt uint64 `json:"SumAt,omitempty"`
    // 重置间隔
    SumInterval uint64 `json:"SumInterval,omitempty"`
    // 下次重制
    SumResetNext uint64 `json:"SumResetNext,omitempty"`
    // 求用户的最近的买入价格的量
    SzForAvg float64 `json:"SzForAvg,omitempty"`
    // Maker最低手续费
    FeeMkrMin gaea_Num.Flt `json:"FeeMkrMin"`
    // Taker最低手续费
    FeeTkrMin gaea_Num.Flt `json:"FeeTkrMin"`
    // 下面是挖矿相关设定
    // 每日有挖矿算力的交易量
    SzMaxFM float64 `json:"SzMaxFM,omitempty"`
    // 每日有挖矿算力的交易次数
    NumMaxFM float64 `json:"NumMaxFM,omitempty"`
    // 涨经验的交易量完成率.当交易量达到 SzMaxFM * ExpRatio Exp ++
    ExpRatio float64 `json:"ExpRatio,omitempty"`
    // 最大Exp
    ExpMax int64 `json:"ExpMax,omitempty"`
    // 标志位
    Flag AssetFlag `json:"Flag,omitempty"`
}

```

补充内测功能：
开始支持虚拟运营的概念，因此在查询取交易对时需要增加参数vp来获取指定虚拟运营商的编号。

```js
// 查询交易对，带参数vp
{"req":"GetAssetD","rid":"0","expires":1537706670830,"args":{"vp":1}}
// 查询交易对扩展定义，带参数vp
{"req":"GetAssetEx","rid":"1","expires":1537706670830,"args":{"vp":1}}
```
内测版返回的交易对属性中会多一个 Lbl 属性，表示该交易的所属板块.


2. 获取综合指数列表： GetCompositeIndex
```js
// 发送请求消息
{
    "req":"GetCompositeIndex",
    "rid":"1",
    "expires":1537706670831,
    "args":{}
}

// 收到返回消息, 成功返回的结构是字符串数组
{
    "rid":"1",
    "code":0,
    "data":["GMEX_CI_ETH","GMEX_CI_BTC"]
}
```

3. 获取历史K线数据： GetHistKLine
```js
// 发送请求消息
{
    "req":"GetHistKLine",
    "rid":"2",
    "expires":1537708009100,
    "args":{
        "Sym":"ETH1812",
        "Typ":"1m",
        "beginSec":1537077600,
        "Offset":0,
        "Count":10
    }
}

// 收到返回消息
{
    "rid":"2",
    "code":0,
    "data":
        {
            "Sym":"ETH1812",
            "Typ":"1m",
            "Count":10,
            "Sec":[1537077600,1537077660,1537077720,1537077780,1537077840,1537077900,1537077960,1537078020,1537078080,1537078140],
            "PrzOpen":[216.45,216,215.8,215.6,215.05,215.25,215.3,215.45,215.5,215.35],
            "PrzClose":[216,215.75,215.6,215,215.35,215.45,215.45,215.7,215.4,215.6],
            "PrzHigh":[216.5,216,215.9,215.6,215.4,215.5,215.45,215.7,215.55,215.6],
            "PrzLow":[215.9,215.65,215.6,214.9,215.05,215.2,215.25,215.4,215.35,215.25],
            "Volume":[1354,717,473,1751,238,269,94,123,123,275],
            "Turnover":[6.26501568815296,3.321813453440903,2.192467668789991,8.137750477124483,1.1054031648919223,1.2493419094774725,0.4364373010900492,0.5702923655266671,0.5709143037474378,1.276362876440699]
    }
}
```

当前系统支持的K线类型有:
```js
/**
 *
 * 类型: 1m, 3m, 5m, 15m, 30m, 1h, 2h, 4h, 6h, 8h, 12h, 1d, 3d, 1w, 2w, 1M
 * m: 代表分钟(minutes)
 * h: 代表小时(hours)
 * d: 代表天(days)
 * w: 代表周(weeks)
 * M: 代表月(months)
 *
 **/
```

获取最近K线数据的指令: GetLatestKLine
```js
// 比如要查询交易对 EOS.USDT 最近5个 [1分钟k] 的数据，请求消息如需：
{"req":"GetLatestKLine","rid":"2","expires":1564109336120,"args":{"Sym":"EOS.USDT","Typ":"1m","Count":5}}

// 返回结果和GetHistKLine的格式一样如下：
{"rid":"2","code":0,"data":
    {
        "Sym":"EOS.USDT","Typ":"1m","Count":5,
        "Sec":[1564109280,1564109220,1564109160,1564109100,1564109040],
        "PrzOpen":[4.501,4.501,4.501,4.501,4.5],
        "PrzClose":[4.502,4.502,4.503,4.5,4.5],
        "PrzHigh":[4.505,4.503,4.505,4.503,4.503],
        "PrzLow":[4.5,4.5,4.5,4.5,4.5],
        "Volume":[2795,1701,1228,1902,1071],
        "Turnover":[62918.45,38288.97,27648.495,42806.005,24100.275]
    }
}
```

4. 订阅/取消订阅: Sub / UnSub
```js
// 发送订阅请求消息
{
    "req":"Sub",
    "rid":"20",
    "expires":1537708219903,
    "args":["tick_BTC1812"]
}

// 收到订阅返回消息
{
    "rid":"20",
    "code":0,
    "data":"OK"
}

// 收到推送消息
{
    "subj":"tick",
    "data":{
        "Sym":"BTC1812",
        "At":1537708218726,
        "PrzBid1":6723.5,
        "SzBid1":429,
        "SzBid":876837,
        "PrzAsk1":6725.5,
        "SzAsk1":494,
        "SzAsk":1022136,
        "LastPrz":6725,
        "SettPrz":6723.98,
        "Prz24":6678.5,
        "High24":6774,
        "Low24":6632,
        "Volume24":5659148,
        "Turnover24":843.7992005395208,
        "Volume":40901214,
        "Turnover":0,
        "OpenInterest":3436394,
    "FundingLongR":0,             // 当前周期内的资金费率
    "FundingPredictedR":0,        // 下个周期预测的资金费率
    "FundingShortR":0             // 当前未使用字段
    }
}

// 发送取消订阅请求消息
{
    "req":"UnSub",
    "rid":"21",
    "expires":1537708267910,
    "args":["tick_BTC1812"]
}

// 收到的取消订阅返回的消息
{
    "rid":"21",
    "code":0,
    "data":"OK"
}
```

##### 用户可以订阅的内容有如下：

| 订阅内容 | 描述 |
| :-----   | :-----  |
|TICK|比如: tick_BTC.BTC, tick行情当前是没500毫秒更新推送一次.|
|成交|比如: trade_BTC.BTC, 所有成交都是实时推送的.|
|20档深度|比如: order20_BTC.BTC 盘口20档深度行情每200毫秒推送一次.|
|全档深度|比如: orderl2_BTC1.BTC 全深度盘口行情订阅后会先推送全档口,后继推送变更档,推送频率100毫秒.|
|K线|比如: kline_1m_BTC1812，kline_1h_BTC.BTC 默认订阅的K线数据推送频率1500毫秒.|
|指数|比如: index_GMEX_CI_BTC，index_GMEX_CI_ETH 订阅的综合指数推送频率为1000毫秒.|

**NOTE**: UnSub 时可以用 * 一次清空, Sub 时必须提供合法的名字.


##### 慢速/快速带宽模式说明

为了节约网络流量，特别提供慢速模式，此时tick会1500毫秒推一次，order20会1000毫秒推一次.
启用慢速模式很简单，只需在sub指令的参数列表中增加一行"\_\_slow\_\_"，如下：

```JavaScript
// 发送订阅请求消息
{
    "req":"Sub",
    "rid":"21",
    "expires":1537708229903,
    "args":["tick_BTC.USDT","order20_BTC.USDT","__slow__"]
}
```

如需恢复正常，则增加"\_\_fast\_\_"即可切换会正常模式。
注意这里的设定是全局的，每次sub时修改了模式，则前面所有订阅涉及到的都会调整速度.


5. 获取服务器时间
```js
// 发送请求消息， 由于本消息开销很小，可用于和服务器端保持网络连接用，比如每隔55秒发送一次；
{
    "req":"Time",
    "rid":"6",
    "expires":1537706745839,
    "args":1537706744839
}
// 收到返回消息
{
    "rid":"6",
    "code":0,
    "data":{
        "time":1537706745295,
        "data":"1537706744839"
    }
}
```


## 交易API

1. 用户登录

用户首先要在网站上的个人中心开启API-KEY功能并生成需要的公私秘钥才能使用交易API功能。

```js
// 发送用户的登录消息

/**
 *  参数说明
 * {
 *  "req":"Login",              			// 请求的动作类型
 *  "rid":"1",
 *  "expires":1538222696758,    			// 消息超时时间
 *  "args":{                    			// 服务端所需的参数
 *      "UserName":"example@gaea.com",              	// 账号
 *      "UserCred":"mVAAADjNHzhvehaEvU$BMJoU7BZk"   	// APIKey
 *  },
 *  "signature": "74c33368e9a1f8d6d13cdf0bf5aa02a8" 	// 签名,可参考生产签名的方法
 * }
 *
 * 生成签名的方法:
 * 公式: md5(Req+rid+Args+Expires+API.SecretKey)
 * 例:
 *  UserName: "example@gaea.com",
 *  UserCred: "mVAAADjNHzhvehaEvU$BMJoU7BZk"
 *  SecretKey: "uLgAAHMw62di3hUPypuETMWGzHx852swxM7V0b2HObba5gYNNrLkuvQ4I"
 *  Req: "Login"
 *  rid: 1
 * 根据上面的信息可以生成如下签名:
 * signature = md5("Login"+"1"+ JSON.stringify({UserName:"example@gaea.com",UserCred:"mVAAADjNHzhvehaEvU$BMJoU7BZk"}) +"1538222696758" + "uLgAAHMw62di3hUPypuETMWGzHx852swxM7V0b2HObba5gYNNrLkuvQ4I")
 * signature结果为:"74c33368e9a1f8d6d13cdf0bf5aa02a8"
 ***/

// 需要注意的是: Args 参数一般为JSON对象(除Time)，在签名时需要序列化为字符串，序列化没有字段顺序要求,但是需要保持签名时序列化的顺序与最终发出消息时序列化的顺序一致。
// 补充: Time消息不要签名

// 发送消息
  {
   "req":"Login",                               // 请求的动作类型
   "rid":"1",
   "expires":1538222696758,                     // 消息超时时间
   "args":{                                     // 服务端所需的参数
       "UserName":"example@gaea.com",           // 账号
       "UserCred":"mVAAADjNHzhvehaEvU$BMJoU7BZk"    // APIKey
   },
   "signature": "74c33368e9a1f8d6d13cdf0bf5aa02a8"  // 签名,可参考生产签名的方法
 }


// 收到返回消息
// 注意：这里收到的 UserId 是用户的系统内部唯一编号，简称 UID， 非常重要，系统用此ID后面增加两位数字表示用户的子账户ID,比如UID=1234567，则合约子账号的AId即为123456701；

{
    "rid":"0",
    "code":0,
    "data":{
        "UserName":"gmex-test@gmail.com",
        "UserId":"1234567"
    }
}
```

用户成功登录交易系统后，所有用户相关信息会自动推送给用户，如报单变更，仓位变化，成交通知，钱包日志等等。
交易的消息定义和行情类似，但多了签名字段：

|参数| 描述|
| :-----   | :-----   |
|req|用户的请求操作动作，如： GetAssetD, GetWallets, GetTrades, GetOrders, GetPositions, OrderNew, OrderDel等等。|
|rid|用户发送请求的唯一编号，由于websocket是异步通讯，用户需要通过匹配收到消息的rid和自己发送的rid来匹配操作和应答。|
|expires|消息超时，毫秒，建议每次发送请求时填写当前时间加1秒。|
|args|用户的参数，可选，具体根据req来设置。|
|signature|消息签名: MD5(Req+ReqId+Args+Expires+API.SecretKey)，小写。|


# 重要说明

+ UID 和 AID
    一个用户对应一个系统内唯一的UID(字符串);

    一个用户下面可以有多个AID,AID是UID后面加两位构成;

    每个用户注册出来时会自动创建一个默认的AID,就是UID后面加01;

    用户可以自己创建多个子账户,因此,一个用户会有多个AID;在系统内部,所有的AID全局唯一;

    用户可以在自己的子账户之间相互转移自己的所有的数字货币;

    用户下单时指定自己的子账户,该单的风险可以控制在这个子账户范围内,从而可以控制风险.

+ 钱包分为合约钱包、币币钱包、资金中心钱包(有时称为我的钱包)三种类型, 通常操作合约钱包时需要使用AId为UID+'01'，操作币币钱包时需要使用AId为UID+'02'，
操作资金中心钱包时则不需要这个参数。


2. 查询当前系统的合约列表(必须参数 AId)： GetAssetD

```js
// 发送请求消息
{
    "req":"GetAssetD",
    "rid":"2",
    "expires":1537710766358,
    "args":{
        "AId":"1525354501"
    },
    "signature": "1234567890abcdef1234567890abcdef"
}

// 收到返回消息
{
    "rid":"2",
    "code":0,
    "data":[
        {
            "Sym":"ETH1812",
            "Beg":1,
            "Expire":1545984000000,
            "PrzMaxChg":1000,
            "PrzMinInc":0.05,
            "PrzMax":1000000,
            "OrderMaxQty":10000000,
            "OrderMinQty":1,
            "LotSz":1,
            "PrzM":245.330000000000012505552149377763271331787109375,
            "MIR":0.07,
            "MMR":0.05,
            "PrzLatest":245.35,
            "OpenInterest":2181137,
            "PrzIndex":244.9823,
            "PosLmtStart":10000000,
            "FeeMkrR":-0.0003,
            "FeeTkrR":0.0007,
            "Mult":1,
            "FromC":"ETH",
            "ToC":"USD",
            "TrdCls":2,
            "MkSt":1,
            "Flag":1,
            "SettleCoin":"ETH",
            "QuoteCoin":"ETH",
            "SettleR":0.0005,
            "DenyOpenAfter":1545980400000
        },
        {"Sym":"BTC1812","Beg":1,"Expire":1545984000000,"PrzMaxChg":1000,"PrzMinInc":0.5,"PrzMax":1000000,"OrderMaxQty":10000000,"LotSz":1,"PrzM":6725.75,"MIR":0.07,"MMR":0.05,"PrzLatest":6724.5,"OpenInterest":3431245,"PrzIndex":6729.2552,"PosLmtStart":10000000,"FeeMkrR":-0.0003,"FeeTkrR":0.0007,"Mult":1,"FromC":"BTC","ToC":"USD","TrdCls":2,"MkSt":1,"Flag":1,"SettleCoin":"BTC","QuoteCoin":"BTC","SettleR":0.0005,"DenyOpenAfter":1545980400000},
        {"Sym":"ETH1809","Beg":1,"Expire":1538121600000,"PrzMaxChg":1000,"PrzMinInc":0.05,"PrzMax":1000000,"OrderMaxQty":10000000,"LotSz":1,"PrzM":244.650000000000005684341886080801486968994140625,"MIR":0.07,"MMR":0.05,"PrzLatest":244.65,"OpenInterest":4501232,"PrzIndex":244.9823,"PosLmtStart":10000000,"FeeMkrR":-0.0003,"FeeTkrR":0.0007,"Mult":1,"FromC":"ETH","ToC":"USD","TrdCls":2,"MkSt":1,"Flag":1,"SettleCoin":"ETH","QuoteCoin":"ETH","SettleR":0.0005,"DenyOpenAfter":1538118000000},
        {"Sym":"BTC1809","Beg":1,"Expire":1538121600000,"PrzMaxChg":1000,"PrzMinInc":0.5,"PrzMax":1000000,"OrderMaxQty":10000000,"LotSz":1,"PrzM":6726.8000000000001818989403545856475830078125,"MIR":0.07,"MMR":0.05,"PrzLatest":6724.0,"OpenInterest":1449455,"PrzIndex":6729.2552,"PosLmtStart":10000000,"FeeMkrR":-0.0003,"FeeTkrR":0.0007,"Mult":1,"FromC":"BTC","ToC":"USD","TrdCls":2,"MkSt":1,"Flag":1,"SettleCoin":"BTC","QuoteCoin":"BTC","SettleR":0.0005,"DenyOpenAfter":1538118000000}
    ]
}
```

返回的结果和行情中获取到的数据是一样的。注意这里必须使用AId，因此合约和币币是分开来获取到的。

```golang

// **交易对/合约的结构定义**
type AssetD struct {
    // 符号 XBTUSD , XBTM18
    Sym string `json:"Sym,omitempty"`
    // 开始时间
    Beg int64 `json:"Beg,omitempty"`
    // 到期日期 永续
    Expire int64 `json:"Expire,omitempty"`
    // //////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // 市价委托的撮合的最多次数。比如5
    PrzMaxChg int32 `json:"PrzMaxChg,omitempty"`
    // 最小的价格变化	0.5 USD
    PrzMinInc gaea_Num.Flt `json:"PrzMinInc"`
    // 最大委托价格	1,000,000
    PrzMax gaea_Num.Flt `json:"PrzMax"`
    // 最大委托数量	10,000,000
    OrderMaxQty gaea_Num.Flt `json:"OrderMaxQty"`
    // 最小合约数量	这个就是每次买卖的合约数量必须是LotSz的倍数。
    LotSz gaea_Num.Flt `json:"LotSz"`
    // 保证金计算相关参数 开始
    // 	标记价格	8103.14
    PrzM gaea_Num.Flt `json:"PrzM"`
    // double LiqR = 10;	//Unused 已废弃。  从 PrzLiq 向 PrzBr偏离的百分比， 0 表示，= PrzLiq, 1 表示 到达PrzBr
    // //////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // 起始保证金	 1.00% + 开仓佣金 + 平仓佣金 Mgn Initial Ratio
    MIR gaea_Num.Flt `json:"MIR"`
    // 维持保证金	0.50% + 平仓佣金 + 资金费率 Mgn Maintaince Ratio
    MMR gaea_Num.Flt `json:"MMR"`
    // ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // 风险限额	200 XBT
    // Unused int64 RiskLimit = 13;
    // 风险限额递增值	100 XBT
    // Unused int64 RiskStep = 14;
    // 标记方法	Prz Mark Method
    PrzMMethod string `json:"PrzMMethod,omitempty"`
    // 合理基差	-12.36
    PrzMFairBasis int64 `json:"PrzMFairBasis,omitempty"`
    // 合理基差率	-410%
    PrzMFairBasisRate int64 `json:"PrzMFairBasisRate,omitempty"`
    // 合理基差计算公式	此合约的合理基差取决于年化资金费率。
    PrzMFairBasisCalc int32 `json:"PrzMFairBasisCalc,omitempty"`
    // ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // 委托的最小价值. 废弃字段。
    OrderMinVal gaea_Num.Flt `json:"OrderMinVal"`
    // 保证金计算相关参数 结束
    // //////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // //////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // 统计信息：
    // 最新成交价格
    PrzLatest gaea_Num.Flt `json:"PrzLatest"`
    // 保证金计算相关参数 结束
    // 最新的成交方向
    DirLatest OrderDir `json:"DirLatest,omitempty"`
    // 总交易量	30,585,913,058
    TotalVol float64 `json:"TotalVol,omitempty"`
    // 持仓量	99,192,762
    OpenInterest int64 `json:"OpenInterest,omitempty"`
    // 总成交额	26,293.1141 XBT
    Turnover float64 `json:"Turnover,omitempty"`
    // 增加指数价格
    PrzIndex gaea_Num.Flt `json:"PrzIndex"`
    // 类型	结算货币为 XBT，计价货币为 USD
    // 	int32 SettleUnit = 25; 未使用。
    // 合约大小	1 USD (目前每张合约价值 0.00012341 XBT)，注意到，是以计价货币来表示的
    AssetSz int64 `json:"AssetSz,omitempty"`
    // 结算	此合约为永续无结算合约。
    // 或者 此合约在 6月29日 下午8:00 (UTC 下午12:00:00) 按照 .BXBT30M 指数 价格结算。
    // 结算日期
    // int64 SettleTime = 27;  与 Expire同义
    // //////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    PosLmtStart int64 `json:"PosLmtStart,omitempty"`
    // 个人占用开仓
    // 当前涨跌价格范围 Prz Rise Fall Range
    PrzRFMin float64 `json:"PrzRFMin,omitempty"`
    // 当前涨跌价格范围最大值
    PrzRFMax float64 `json:"PrzRFMax,omitempty"`
    // 佣金费率
    // 提供流动性的费率	FeeMkrR
    FeeMkrR gaea_Num.Flt `json:"FeeMkrR"`
    // 消耗流动性的费率
    FeeTkrR gaea_Num.Flt `json:"FeeTkrR"`
    // Order中，Qty必须是Mult的倍数
    Mult gaea_Num.Flt `json:"Mult"`
    // 从什么货币 购买行为消耗的货币符号
    FromC string `json:"FromC,omitempty"`
    // 兑换为 什么货币  购买行为得到的货币符号
    ToC string `json:"ToC,omitempty"`
    // 交易类型
    TrdCls TradeClass `json:"TrdCls,omitempty"`
    // 市场状态
    MkSt MkStatus `json:"MkSt,omitempty"`
    // 标志, 正向报价，反向报价
    Flag AssetFlag `json:"Flag,omitempty"`
    // 结算货币
    SettleCoin string `json:"SettleCoin,omitempty"`
    // 报价货币
    QuoteCoin string `json:"QuoteCoin,omitempty"`
    // 结算费率
    SettleR gaea_Num.Flt `json:"SettleR"`
    // 时间节点：当越过了DenyOpenAfter后，不允许开新仓
    DenyOpenAfter int64 `json:"DenyOpenAfter,omitempty"`
    // 如果允许使用第三种货币支付手续费，则配置本项目
    FeeCoin string `json:"FeeCoin,omitempty"`
    // 如果允许使用第三种货币支付手续费，这里配置折扣率
    FeeDiscR gaea_Num.Flt `json:"FeeDiscR"`
    // 最大委托数量	10,000,000
    OrderMinQty gaea_Num.Flt `json:"OrderMinQty"`
    // //////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // 永续合约专属数据
    // 基础货币利率符号	.XBTBON8H
    InterestBaseSym string `json:"InterestBaseSym,omitempty"`
    // 计价货币利率符号	.USDBON8H
    InterestQuoteSym string `json:"InterestQuoteSym,omitempty"`
    // 资金费用溢价符号	.XBTUSDPI8H
    FundingPremiumSym string `json:"FundingPremiumSym,omitempty"`
    // 资金费率	-0.3750%
    // 多仓资金费率
    FundingLongR float64 `json:"FundingLongR,omitempty"`
    // 空仓资金费率
    FundingShortR float64 `json:"FundingShortR,omitempty"`
    // 资金费用收取间隔 秒。	每 8 小时 //秒，8小时 = 8 * 3600 秒 = 28800 秒 = 28800000 毫秒
    FundingInterval uint32 `json:"FundingInterval,omitempty"`
    // 下一个资金费率结算的时间。2018年4月16日 下午8:00:00. 时间戳 毫秒
    FundingNext int64 `json:"FundingNext,omitempty"`
    // 预测费率	-0.3586%
    FundingPredictedR float64 `json:"FundingPredictedR,omitempty"`
    // 每日0点后的 FundingOffset 毫秒后 为第一个结算时间点
    FundingOffset int64 `json:"FundingOffset,omitempty"`
    // 资金费率计算参数: 公差
    FundingTolerance float64 `json:"FundingTolerance,omitempty"`
    // Funding结算佣金
    FundingFeeR gaea_Num.Flt `json:"FundingFeeR"`
    // 交易对分组. 比如分组
    Grp int64 `json:"Grp,omitempty"`
}

// **定义手续分的收取方式**
type FeeMethod int32

const (
    FeeMethod_FM_IN_FROM_TO         FeeMethod = 0
    FeeMethod_FM_IN_FROM            FeeMethod = 1
    FeeMethod_FM_IN_FROM_TO_FEECOIN FeeMethod = 2
    FeeMethod_FM_IN_FROM_FEECOIN    FeeMethod = 3
)

// **交易对的扩展配置数据**
type AssetEx struct {
    // 合约符合/交易对符号
    Sym  string  `json:"Sym"`
    // 手续费计费方法
    FM FeeMethod `json:"FM,omitempty"`
    // 手续费，货币符号，如果未指定，则现货：按照收入额进行收取。期货：按照SettleCoin进行。
    // 如果指定了FeeCoin则从该币种钱包内进行扣除。注意到，如果该钱包余额不足，则依旧使用SettleCoin进行
    FeeCoin string `json:"FeeCoin,omitempty"`
    // 折扣率
    FeeDiscR float64 `json:"FeeDiscR"`
    // 开放交易时间 (日内,毫秒)
    OnAt uint64 `json:"OnAt,omitempty"`
    // 关闭交易时间 (日内,毫秒)
    OffAt uint64 `json:"OffAt,omitempty"`
    // 价格涨价幅度 万分比 * 10000
    RiseR int64 `json:"RiseR,omitempty"`
    // 价格跌价幅度 万分比 * 10000
    FallR int64 `json:"FallR,omitempty"`
    // 最小价格
    PrzMin float64 `json:"PrzMin,omitempty"`
    // 买入量
    LmtBid float64 `json:"LmtBid,omitempty"`
    // 卖出量
    LmtAsk float64 `json:"LmtAsk,omitempty"`
    // 买入卖出总量
    LmtBidAsk float64 `json:"LmtBidAsk,omitempty"`
    // 买入次数
    LmtNumBid uint64 `json:"LmtNumBid,omitempty"`
    // 卖出次数
    LmtNumAsk uint64 `json:"LmtNumAsk,omitempty"`
    // 买入卖出总次数
    LmtNumBidAsk uint64 `json:"LmtNumBidAsk,omitempty"`
    // 委托的买价偏离盘口比例(小数)
    BidPrzR float64 `json:"BidPrzR,omitempty"`
    // 委托的卖价偏离盘口比例(小数)
    AskPrzR float64 `json:"AskPrzR,omitempty"`
    // 从0点开始，在每天的什么时间，开始重置统计值(绝对时间,毫秒)
    SumAt uint64 `son:"SumAt,omitempty"`
    // 重置间隔
    SumInterval uint64 `json:"SumInterval,omitempty"`
    // 下次重制
    SumResetNext uint64 `json:"SumResetNext,omitempty"`
}

```


3. 查询用户子账号的钱包列表信息： GetWallets 和 GetCcsWallets

用户在交易所中的钱包分为 资金中心钱包(我的钱包)，合约钱包，币币钱包 三种，每个用户都会有这个三个钱包的数据。
资金中心的钱包的信息数据结构和另外两个有较大不同，因此单独定义之，合约钱包和币币钱包的数据结构一样。
简言之：
- 查询合约钱包： GetWallets （AId=UID+01）
- 查询币币钱包： GetWallets （AId=UID+02）
- 查询资金中心钱包： GetCcsWallets

```js
// 示例： GetWallets
// 发送请求消息
{
    "req":"GetWallets",
    "rid":"3",
    "expires":1537710967223,
    "args":{
        "AId":"1525354501"
    },
    "signature": "1234567890abcdef1234567890abcdef"
}
// 收到返回消息
{
    "rid":"3",
    "code":0,
    "data":[
        {
            "UId":"1234567",
            "AId":"123456701",
            "Coin":"ETH",
            "Depo":1.00000000,
            "WDrw":0,
            "PNL":-0.452374713185744188799864398523351731769754619710,
            "Frz":0,
            "MI":0.2906262618301145,
            "RD":0.530702779487773,
            "Status":2
        },
        {"UId":"1234567","AId":"123456701","Coin":"BTC","Depo":0.16518449,"WDrw":0,"PNL":-0.00426155098522019276893748290683797431047794081854,"Frz":0,"MI":0.04244959641866207,"RD":0.26378834912257804,"Status":2},
        {"UId":"1234567","AId":"123456701","Coin":"GAEA","Depo":5,"WDrw":0,"PNL":0,"Frz":0,"Status":2}
    ]
}
```

资金中心钱包（我的钱包）查询示例：

```js

// 示例： GetCcsWallets，注意返回的结构体和上面的是不一样的。
// 发送请求消息
{
    "req":"GetCcsWallets",
    "rid":"9",
    "expires":1537710967223,
    "signature": "1234567890abcdef1234567890abcdef"
}
// 收到返回消息
{
    "rid":"9",
    "code":0,
    "data":[
        {
            "wid": "1020415BTC",                        // 主键：资金账户id，uid+Wtype
            "uid": "1020415",                           // 用户Id
            "coin": "BTC",                              // 币种的名称
            "mainBal": 3.06,                            // 主账户余额
            "otcBal": 0,                                // OTC法币账户余额
            "lockBal": 0,                               // 锁币额度
            "financeBal": 0,                            // 理财额度
            "pawnBal": 0,                               //质押额度
            "creditNum": 0                              // 欠贷款额度【负】
        },
        {
            "wid": "1020415ETH",                        // 主键：资金账户id，uid+Wtype
            "uid": "1020415",                           // 用户Id
            "coin": "ETH",                              // 币种的名称
            "mainBal": 8.16,                            // 主账户余额
            "otcBal": 0,                                // OTC法币账户余额
            "lockBal": 0,                               // 锁币额度
            "financeBal": 0,                            // 理财额度
            "pawnBal": 0,                               //质押额度
            "creditNum": 0                              // 欠贷款额度【负】
        }
    ]
}
```



4. 查询用户子账号的最近的成交记录(必须参数 AId)： GetTrades
```js
// 发送请求消息
{
    "req":"GetTrades",
    "rid":"4",
    "expires":1537711037271,
    "args":{
        "AId":"123456701"
    },
    "signature": "1234567890abcdef1234567890abcdef"
}

// 收到返回消息, 默认最多返回200条记录，通过在args中增加设置参数("Start"=0,"Stop"=500)可以最多返回500条记录.
{
    "rid":"4",
    "code":0,
    "data":[
        {
            "UId":"1234567",
            "AId":"123456701",
            "Sym":"BTC1812",
            "MatchId":"01CQES0XMV1WWWXP63PMPKE9RF",
            "OrdId":"01CQES0XMV4M3XNJBXKBCKHPZ3",
            "Sz":259,
            "Prz":6.75E+3,
            "Fee":-0.00001151111111111111,
            "FeeCoin":"BTC",
            "At":1537703229547,
            "Via":7
        },
        {"UId":"1234567","AId":"123456701","Sym":"BTC1812","MatchId":"01CQES0XMVKDQCD192BG4STJF6","OrdId":"01CQES0XMV4M3XNJBXKBCKHPZ3","Sz":519,"Prz":6.75E+3,"Fee":-0.00002306666666666667,"FeeCoin":"BTC","At":1537703229346,"Via":7},
        {"UId":"1234567","AId":"123456701","Sym":"BTC1812","MatchId":"01CQES0XMVXD7ZPV22YF49AW93","OrdId":"01CQES0XMV4M3XNJBXKBCKHPZ3","Sz":504,"Prz":6.75E+3,"Fee":-0.00002240000000000000,"FeeCoin":"BTC","At":1537703229146,"Via":7},
        {"UId":"1234567","AId":"123456701","Sym":"BTC1812","MatchId":"01CQES0XMV9FDYBTRX29VZ1WAP","OrdId":"01CQES0XMV4M3XNJBXKBCKHPZ3","Sz":585,"Prz":6.75E+3,"Fee":-0.00002600000000000000,"FeeCoin":"BTC","At":1537703228944,"Via":7}
    ]
}
```

5. 查询用户最长的当前有效的报单列表(必须参数 AId)： GetOrders
```js
// 发送请求消息
{
    "req":"GetOrders",
    "rid":"5",
    "expires":1537711298635,
    "args":{
        "AId":"123456701"
    },
    "signature": "1234567890abcdef1234567890abcdef"
}
// 收到返回消息
{
    "rid":"5",
    "code":0,
    "data":[
        {
            "UId":"1234567",
            "AId":"123456701",
            "Sym":"ETH1812",
            "OrdId":"01CQES0XMVA1CWXJ823B8TV9FA",
            "COrdId":"1537703785880",
            "Dir":-1,
            "OType":1,
            "Prz":265,
            "Qty":1000,
            "QtyDsp":0,
            "PrzStop":0,
            "At":1537703785905,
            "Upd":1537703785905,
            "Until":9223372036854775807,
            "Frz":0,
            "Status":2,
            "QtyF":0,
            "PrzF":0,
            "Val":0,
            "StopPrz":0
        },
        {"UId":"1234567","AId":"123456701","Sym":"BTC1812","OrdId":"01CQES0XMV4FKJMJFFC8SC4EE3","COrdId":"1537703873308","Dir":-1,"OType":1,"Prz":7.10E+3,"Qty":4000,"QtyDsp":0,"PrzStop":0,"At":1537703873327,"Upd":1537703873328,"Until":9223372036854775807,"Frz":0,"Status":2,"QtyF":0,"PrzF":0,"Val":0,"StopPrz":0}
    ]
}
```

6. 查询用户子账号的当前持仓列表(必须参数 AId)： GetPositions
```js
// 发送请求消息
{
    "req":"GetPositions",
    "rid":"6",
    "expires":1537711980986,
    "args":{
        "AId":"123456701"
    },
    "signature": "1234567890abcdef1234567890abcdef"
}

// 收到返回消息
{
    "rid":"6",
    "code":0,
    "data":[
        {
            "UId":"1234567",
            "PId":"01CQES0XMVZ2T6GBWVBX62TMMK",
            "AId":"123456701",
            "Sym":"ETH1812",
            "Sz":100,
            "PrzIni":246.75,
            "RPNL":0.000121580547112462,
            "Val":0.4061408496466575,
            "MMnF":0.020307042482332876,
            "MI":0.26066975804143216,
            "UPNL":-0.0008723592717840498,
            "PrzLiq":154.1425703534361,
            "PrzBr":146.7046448590807,
            "FeeEst":0.0002842985947526602,
            "ROE":-0.04236534514766641,
            "ADLIdx":-0.07944234516057963
        },
        {"UId":"1234567","PId":"01CQES0XMVJGCXCS0MF1P2ZK5V","AId":"123456701","Sym":"BTC1809","Sz":-120,"PrzIni":6737.5,"RPNL":0.000005343228200371059,"Val":0.017813378143875687,"MMnF":0.0008906689071937843,"UPNL":0.0000026174759721611044,"PrzLiq":63949689.43002453,"PrzBr":67365100.00002584,"FeeEst":0.000012469364700712979,"ROE":0.0028982007003982577,"ADLIdx":0.0007826905463650653,"ADLLight":3},
        {"UId":"1234567","PId":"01CQES0XMVS6XK7P4W46ZTY17H","AId":"123456701","Sym":"ETH1809","Sz":50,"PrzIni":245.55,"RPNL":0.00006108735491753208,"Val":0.20377389248889433,"MMnF":0.010188694624444716,"UPNL":-0.00014937609712074688,"PrzLiq":111.97759051807083,"PrzBr":106.57427478640034,"FeeEst":0.000142641724742226,"ROE":-0.014458545542610519,"ADLIdx":-0.027112272106186153}
    ]
}
```

7. 查询用户子账号的最近已完成的报单列表(必须参数 AId)： GetHistOrders
```js
// 发送请求消息
{
    "req":"GetHistOrders",
    "rid":"7",
    "expires":1537712072667,
    "args":{
        "AId":"123456701"
    },
    "signature": "1234567890abcdef1234567890abcdef"
}

// 收到返回消息
{
    "rid":"7",
    "code":0,
    "data":[
		{"AId":"123456701","At":1551057354296,"COrdId":"1551057354239","Dir":1,"Frz":0,"OrdId":"01D16QEQFCHP3JH4VA3FYFTDEN","OType":1,"Prz":3700,"PrzF":3700,"PrzStop":0,"Qty":60,"QtyDsp":0,"QtyF":60,"Status":4,"StopPrz":0,"Sym":"BTC1903","UId":"1234567","Until":9223372036853775000,"Upd":1551299771782,"Val":0,"WId":"123456701BTC"},
		{"AId":"123456701","At":1551057304192,"COrdId":"1551057304136","Dir":1,"Frz":0,"OrdId":"01D16QEQFC08QK9N655H2NETKE","OType":2,"Prz":3790,"PrzChg":10,"PrzF":3790,"PrzStop":0,"Qty":50,"QtyDsp":0,"QtyF":50,"Status":4,"StopPrz":0,"Sym":"BTC1903","Tif":1,"UId":"1234567","Until":9223372036853775000,"Upd":1551057304192,"Val":0,"WId":"123456701BTC"},
		{"AId":"123456701","At":1550765634218,"COrdId":"1550765634165","Dir":-1,"Frz":0,"OrdId":"01D16QEQFCVNA8KG86DKZDRKQC","OType":1,"Prz":3939,"PrzF":3939,"PrzStop":0,"Qty":50,"QtyDsp":0,"QtyF":50,"Status":4,"StopPrz":0,"Sym":"BTC1903","UId":"1234567","Until":9223372036853775000,"Upd":1550800893882,"Val":0,"WId":"123456701BTC"},
		{"AId":"123456701","At":1550765301943,"COrdId":"1550765301926","Dir":-1,"ErrCode":27,"Frz":0,"OrdId":"01D16QEQFCXFBB21QJ32SHWVHG","OType":1,"Prz":3939.5,"PrzF":0,"PrzStop":0,"Qty":50,"QtyDsp":0,"QtyF":0,"Status":4,"StopPrz":0,"Sym":"BTC1903","UId":"1234567","Until":9223372036853775000,"Upd":1550765362567,"Val":0.01269196598,"WId":"123456701BTC"},
		{"AId":"123456701","At":1550635738299,"COrdId":"1550635738201","Dir":-1,"Frz":0,"OrdId":"01D16QEQFC6E1WN77PRWG8DGRZ","OType":2,"Prz":3913,"PrzChg":10,"PrzF":3913,"PrzStop":0,"Qty":40,"QtyDsp":0,"QtyF":40,"Status":4,"StopPrz":0,"Sym":"BTC1903","Tif":1,"UId":"1234567","Until":9223372036853775000,"Upd":1550635738299,"Val":0,"WId":"123456701BTC"},
		{"AId":"123456701","At":1548655357742,"Dir":-1,"Frz":0,"OrdId":"01D16QEQFCVF74KVQHQ6WAD2ZP","OType":1,"Prz":103.85,"PrzF":106.05,"PrzStop":0,"Qty":1,"QtyDsp":0,"QtyF":1,"Status":4,"StopPrz":0,"Sym":"ETH1903","Tif":2,"UId":"1234567","Upd":1548655357742,"Val":0,"Via":4,"WId":"123456701ETH"}
	]
}
```


8. 查询用户子账号的最近钱包日志列表(必须参数 AId)： GetWalletsLog
```js

// 发送请求消息
{
    "req":"GetWalletsLog",
    "rid":"9",
    "expires":1537712200855,
    "args":{
        "AId":"123456701"
    },
    "signature": "1234567890abcdef1234567890abcdef"
}

// 收到返回消息
{
    "rid":"9",
    "code":0,
    "data":[
        {
            "UId":"1234567",
            "AId":"123456701",
            "Seq":"01CQES0XMVR78FS73QV5WBT3F8",
            "Coin":"BTC",
            "Qty":0.000005343228200371059,
            "Fee":0,
            "WalBal":0.1609282822429802,
            "At":1537711867713,
            "Op":3,
            "Via":8,
            "Info":"Fee :01CQES0XMV4VFBEETKJ2522BKH",
            "Stat":4,
            "DirtyFlag":11
        },
        {"UId":"1234567","AId":"123456701","Seq":"01CQES0XMVTMQHGPGJZMP9WJKT","Coin":"ETH","Qty":0.00006108735491753208,"Fee":0,"WalBal":0.5478079547162858,"At":1537711603240,"Op":3,"Via":8,"Info":"Fee :01CQES0XMVJKBYW747NX6HKYMV","Stat":4,"DirtyFlag":11},
        {"UId":"1234567","AId":"123456701","Seq":"01CQES0XMVYS0W0E1Q69Y9GC3X","Coin":"ETH","Qty":0.0001215805471124620,"Fee":0,"WalBal":0.5477468673613683,"At":1537711597374,"Op":3,"Via":8,"Info":"Fee :01CQES0XMVPEVSRHC5N6F44NVQ","Stat":4,"DirtyFlag":11},
    ]
}
```

9. 获取服务器时间： Time
```js
// 发送请求消息， 由于本消息开销很小，可用于和服务器端保持网络连接用，比如每隔55秒发送一次；
{
    "req":"Time",
    "rid":"13",
    "expires":1537713841078,
    "args":1537713840076
}

// 收到返回消息
{
    "rid":"13",
    "code":0,
    "data":{
        "time":1537713840095,
        "data":"1537713840076"
    }
}
```

10. 下单： OrderNew

```js
// 发送下单请求
// COrdId 是 Client Order ID 的意思，不能为空，由用户生成管理并维护其唯一性(长度不超过40的字符串).
// 当报单成功后，会对应一个OrdId，为系统能够识别的报单编号;
// 注意，用户发起下单后，要通过 onOrder 消息来监控管理报单的状态变化;
{
    "req":"OrderNew",
    "rid":"10",
    "expires":1537712923999,
    "args":{
        "AId":"123456701",
        "COrdId":"c4681144dc5b4051925f00e8339ee97f",
        "Sym":"BTC1809",
        "Dir":1,
        "OType":1,
        "Prz":6500,
        "Qty":2,
        "QtyDsp":0,
        "Tif":0,
        "OrdFlag":0,
        "PrzChg":0
    },
    "signature": "1234567890abcdef1234567890abcdef"
}

// 收到返回消息
{
    "rid":"10",
    "code":0,
    "data":{
        "UId":"1234567",
        "AId":"123456701",
        "Sym":"BTC1809",
        "OrdId":"01CQES0XMVV3SMWJ7N683FWJR8",
        "COrdId":"c4681144dc5b4051925f00e8339ee97f",
        "Dir":1,
        "OType":1,
        "Prz":6500,
        "Qty":2,
        "QtyDsp":0,
        "PrzStop":0,
        "At":1537712923017,
        "Until":9223372036854775807,
        "Frz":0,
        "Status":1,
        "QtyF":0,
        "PrzF":0,
        "Val":0,
        "StopPrz":0
    }
}


// 后继 onOrder 推送消息会汇报报单的变更情况
{
    "subj":"onOrder",
    "data":{
        "WId":"123456701BTC",
        "Prz":6500,
        "Qty":2,
        "Upd":1537712923017,
        "Frz":0,
        "PrzF":0,
        "AId":"123456701",
        "COrdId":"0",
        "QtyDsp":0,
        "PrzStop":0,
        "Val":0,
        "Dir":1,
        "Until":9223372036854776000,
        "Status":2,
        "QtyF":0,
        "StopPrz":0,
        "UId":"1234567",
        "Sym":"BTC1809",
        "OrdId":"01CQES0XMVV3SMWJ7N683FWJR8",
        "OType":1,
        "At":1537712923017
    }
}
```

报单的基本参数说明：
```js
args: {
    "AId": "账户Id",
    "COrdId": "<uuid>", // COrdId 是 Client Order ID 的意思，不能为空，由用户生成管理并维护其唯一性(长度不超过40的字符串).
    "Sym": "BTC1809",   // 交易符号，比如XBTUSD
    "Dir": 1,           // 委单方向 买/卖, 1:BID/BUY, -1:ASK/SELL
    "OType": 1,         // 报价类型, 1:Limit(限价委单 ), 2: Market(市价委单,匹配后转限价), 3: StopMarket (市价止损);
    "Prz": 8000,        // 价格
    "Qty": 10000,       // 数量(如果>0则为做多,如果<0则为做空)
    "QtyDsp": 0,        // 显示数量, 0表示不隐藏, 用于支持冰山委托
    "Tif": 0,           // 生效时间设定, 0:GoodTillCancel, 1:ImmediateOrCancel/FillAndKill, 2:FillOrKill
    "OrdFlag": 0,       // 标志位, 0: OF_INVALID, 1: POSTONLY, 2: REDUCEONLY, 4: CLOSEONTRIGGER;
    "PrzChg" 0,         // 市价成交档位
    // ... 跟多参数，请参考下面的Ord数据结构定义.
}
```
更多关于报单数据结构的的参数定义和说明，请参考下面的推送消息章节里的结构定义。


11. 撤单： OrderDel
```js
// 发送撤单请求
{
    "req":"OrderDel",
    "rid":"11",
    "expires":1537713298949,
    "args":{
        "AId":"123456701",
        "OrdId":"01CQES0XMVV3SMWJ7N683FWJR8",
        "Sym":"BTC1809"
    },
    "signature": "1234567890abcdef1234567890abcdef"
    }
}

// 收到返回消息
{
    "rid":"11",
    "code":0,
    "data":{
        "UId":"1234567",
        "AId":"123456701",
        "Sym":"BTC1809",
        "OrdId":"01CQES0XMVV3SMWJ7N683FWJR8",
        "Prz":0,
        "Qty":0,
        "QtyDsp":0,
        "PrzStop":0,
        "At":1537713297967,
        "Until":9223372036854775807,
        "Frz":0,
        "Status":1,
        "QtyF":0,
        "PrzF":0,
        "Val":0,
        "StopPrz":0
    }
}

// 后继 onOrder 推送消息会汇报报单的变更情况
{
    "subj":"onOrder",
    "data":{
        "WId":"123456701BTC",
        "At":1537713297967,
        "Until":9223372036854776000,
        "Status":1,
        "OrdId":"01CQES0XMVV3SMWJ7N683FWJR8",
        "Prz":0,
        "PrzF":0,
        "Val":0,
        "Frz":0,
        "QtyF":0,
        "UId":"1234567",
        "Sym":"BTC1809",
        "Qty":0,
        "PrzStop":0,
        "AId":"123456701",
        "QtyDsp":0,
        "Upd":1537713297967,
        "StopPrz":0
    }
}
```


12. 设置超时撤单(必须参数 AId)： CancelAllAfter

|参数| 描述|
| :-----   | :-----   |
|AId|用户的子账号ID|
|Sec|设置N秒后自动撤销AId下的所有报单|

调用此接口成功后，用户该AId下的所有报单将在n秒后被全部自动撤单。通过设置0秒可以禁用此功能,常见的使用模式是设 timeout 为 60000,并每隔 15 秒调用一次,建议每次使用完API将Sec设置为0,禁用此功能。


 13. 查询用户的风险限额GetRiskLimit(内测中)
```JavaScript
/**
* 功能: 查询某个交易对用户的风险限额
* 参数说明:
* expires:          // 消息的有效时间
* rid: 10           //用户发送请求的唯一编号，由于websocket是异步通讯，用户需要通过匹配收到消息的rid和自己发送的rid来匹配操作和应答。
* req: 'GetRiskLimit'   // 请求的动作名称
* signature: ""         // 签名,参考签名的生成规则
* args: {
*  "AId": "",           // 账号的AId,
*  "Sym": "",           // 交易对名称
* }
*/
// 请求发送参数
{"req":"GetRiskLimit","rid":"15","expires":1537712072667,"args":{"AId":"123456701","Sym":"BTC1812"},"signature": "1234567890abcdef1234567890abcdef"}
// 接收到的返回消息
{"rid":"15","code":0,"data":{....}}

```

风险限额的定义如下

```golang
    // RiskLimit的数据结构
    type RiskLimitDef struct {
        Sym         string      // Symbol 交易对
        Base        float64     // Base Risk Limit 当 Pos Val < Base 的时候，
        BaseMMR     float64     // Base Maintenance Margin Val < Base 的时候 MMR
        BaseMIR     float64     // Initial Margin  Val < Base 的时候 MIR
        Step        float64     // Step  StepS = math.Ceil((Val - Base)/Step) 表示递增次数
        StepMR      float64     // StepM  每次递增的时候，MMR MIR 的增量
        PosSzMax    float64     // 最大持仓
        StepIR      float64     // 每次递增的时候，MIR 的增量
        MaxOrdVal   float64     // 单笔委托的最大价值
    }
```


14. 更多的查询功能(内测中)

更多查询功能当前内测的有：
    - GetAssetExCfg, 返回结果对应结构 V2AssetCfg;
    - GetExchangeRate，返回结果对应结构 V2ExchangeRate;
    - GetMktSum，返回结果对应结构 V2MktSum;
    - GetTrdSum；返回结果对应结构 V2TrdSum;

以上查询操作的参数args需要AId和Sym。

```golang

// GetExchangeRate 返回的结构定义：
type V2ExchangeRate struct {
    // 交易对(合约对) Key
    CoinPair string `json:"CoinPair,omitempty"`
    // 消耗
    FromC string `json:"FromC,omitempty"`
    // 收入
    ToC string `json:"ToC,omitempty"`
    // 买入价
    Buy gaea_Num.Flt `json:"Buy"`
    // 卖出价
    Sell gaea_Num.Flt `json:"Sell"`
    // 更新时间戳(毫秒)
    Upd int64 `json:"Upd,omitempty"`
    // 标志位
    Flag AssetFlag `json:"Flag,omitempty"`
}

// GetMktSum 返回的结构定义：
type V2MktSum struct {
    // 合约ID
    Sym string `json:"Sym,omitempty"`
    // 开多仓量
    PosL int64 `json:"PosL,omitempty"`
    // 开多价值
    ValL float64 `json:"ValL,omitempty"`
    // 开空仓量
    PosS int64 `json:"PosS,omitempty"`
    // 开空价值
    ValS float64 `json:"ValS,omitempty"`
    // UPNL
    UPNL float64 `json:"UPNL,omitempty"`
    // PNL
    PNL float64 `json:"PNL,omitempty"`
    // 用户数
    Usrs int64 `json:"Usrs,omitempty"`
}


// GetTrdSum 返回的结构定义：
type V2TrdSum struct {
    // 账号ID
    AId string `json:"AId,omitempty"`
    // 交易对
    Sym string `json:"Sym,omitempty"`
    // 买入量
    Bid float64 `json:"Bid,omitempty"`
    // 卖出量
    Ask float64 `json:"Ask,omitempty"`
    // 买入次数
    NumBid uint64 `json:"NumBid,omitempty"`
    // 卖出次数
    NumAsk uint64 `json:"NumAsk,omitempty"`
    // 本周期结束的时间戳(毫秒)
    Next uint64 `json:"Next,omitempty"`
    // 最近一次买交易
    LstPB float64 `json:"LstPB,omitempty"`
    // 最近一次买数量
    LstQB float64 `json:"LstQB,omitempty"`
    // 最近一次买交易的时间(毫秒)
    LstTB uint64 `json:"LstTB,omitempty"`
    // 最近一次卖交易价格
    LstPA float64 `json:"LstPA,omitempty"`
    // 最近一次卖交易数量
    LstQA float64 `json:"LstQA,omitempty"`
    // 最近一次卖交易的时间(毫秒)
    LstTA uint64 `json:"LstTA,omitempty"`
    // 平均买入价
    BAvg float64 `json:"BAvg,omitempty"`
    // 买入量
    NBid float64 `json:"NBid,omitempty"`
    // 平均卖出价
    AAvg float64 `json:"AAvg,omitempty"`
    // 卖出量
    NAsk float64 `json:"NAsk,omitempty"`
    // 挖矿算力等级
    MPL int64 `json:"MPL,omitempty"`
    // 挖矿Exp
    MPExp int64 `json:"MPExp,omitempty"`
}

```


15. 用户收到的推送消息

用户登录后会收到的推送消息的subj有：

报单通知 onOrder

持仓通知 onPosition

钱包通知 onWallet

钱包日志 onWltLog

成交通知 onTrade

```js
// 比如：钱包变化
{
    "subj":"onWallet",
    "data":{
        "PNL":-0.004261550985220193,
        "MI":0.04240898874506015,
        "Status":2,
        "Coin":"BTC",
        "WId":"123456701BTC",
        "Depo":0.16518449,
        "Frz":0,
        "RD":0.2635360067663513,
        "UId":"1234567",
        "AId":"123456701",
        "WDrw":0
    }
}
```

对应的数据结构定义如下:

```golang

// 报单数据结构
type Ord struct {
    // /用户Id
    UId string `json:"UId,omitempty"`
    // /账户Id
    AId string `json:"AId,omitempty"`
    // 交易对。比如XBTUSD
    Sym string `json:"Sym,omitempty"`
    // /钱包ID
    WId string `json:"WId,omitempty"`
    // / 服务器端为其分配的ID
    OrdId string `json:"OrdId,omitempty"`
    // / 客户端为其分配的ID
    COrdId string `json:"COrdId,omitempty"`
    // 委单方向 1=买/-1=卖
    Dir OrderDir `json:"Dir,omitempty"`
    // 报价类型
    OType OfferType `json:"OType,omitempty"`
    // 价格
    Prz gaea_Num.Flt `json:"Prz"`
    // 数量。
    Qty gaea_Num.Flt `json:"Qty"`
    // 显示数量。如果为0,则显示全部Qty
    QtyDsp gaea_Num.Flt `json:"QtyDsp"`
    // 有效期
    Tif TimeInForce `json:"Tif,omitempty"`
    // 委托标志
    OrdFlag OrdFlag `json:"OrdFlag,omitempty"`
    // 未使用
    PrzStop gaea_Num.Flt `json:"PrzStop"`
    // 来源
    Via OrderVia `json:"Via,omitempty"`
    // 下单时间戳.单位:毫秒
    At uint64 `json:"At,omitempty"`
    // 更新时间戳.单位:毫秒
    Upd int64 `json:"Upd,omitempty"`
    // 有效期: 毫秒。绝对时间
    Until uint64 `json:"Until,omitempty"`
    // 市价委托的最大档位(当撮合进行匹配的时候，会从Orderbook依档位进行)
    PrzChg int32 `json:"PrzChg,omitempty"`
    // 冻结金额
    Frz gaea_Num.Flt `json:"Frz"`
    // 错误代码
    ErrCode ErrorCode `json:"ErrCode,omitempty"`
    // 错误文本
    ErrTxt string `json:"ErrTxt,omitempty"`
    // 状态
    Status OrderStatus `json:"Status,omitempty"`
    // 已成交 Qty Filled
    QtyF gaea_Num.Flt `json:"QtyF"`
    // 已成交的平均价格 Prz Filled
    PrzF gaea_Num.Flt `json:"PrzF"`
    // 合约价值,对于PRZ_INVERSE的合约：  - Dir * Qty / Prz; 对于正向合约 Dir * Qty * Prz
    Val gaea_Num.Flt `json:"Val"`
    // 仓位Id,如果指定了仓位Id,则本委托导致的的仓位变化，为修改指定的仓位
    PId string `json:"PId,omitempty"`
    // 判断依据
    StopBy StopBy `json:"StopBy,omitempty"`
    // 止损价格,止盈价格
    StopPrz gaea_Num.Flt `json:"StopPrz"`
    // 追踪委托中，回调的比率. Reverse Ratio. 小数。
    TraceRR float64 `json:"TraceRR,omitempty"`
    // 追踪的Min
    TraceMin float64 `json:"TraceMin,omitempty"`
    // 追踪的Max
    TraceMax float64 `json:"TraceMax,omitempty"`
    // //////////////////////////////////////////////////////////////////////////////////////////////////
    // 委托保证金 Mgn Initial + 佣金
    MM float64 `json:"MM,omitempty"`
    // 预估的手续费：按照手续费计算
    FeeEst float64 `json:"FeeEst,omitempty"`
    // 预估的UPNL .. Predicatee
    UPNLEst float64 `json:"UPNLEst,omitempty"`
}

// 用户持仓
type Position struct {
    // 用户Id
    UId string `json:"UId,omitempty"`
    // 仓位Id
    PId string `json:"PId,omitempty"`
    // /AId
    AId string `json:"AId,omitempty"`
    // 交易对/合约名
    Sym string `json:"Sym,omitempty"`
    // 钱包Id
    WId string `json:"WId,omitempty"`
    // 仓位(正数为多仓，负数为空仓)
    Sz gaea_Num.Flt `json:"Sz,omitempty"`
    // 开仓平均价格()
    PrzIni gaea_Num.Flt `json:"PrzIni,omitempty"`
    // 已实现盈亏
    RPNL float64 `json:"RPNL,omitempty"`
    // 杠杆
    Lever float64 `json:"Lever,omitempty"`
    // 逐仓下仓位保证金
    MgnISO gaea_Num.Flt `json:"MgnISO,omitempty"`
    // 逐仓下已实现盈亏
    PNLISO gaea_Num.Flt `json:"PNLISO,omitemptys"`
    // 下面是动态数据
    // 最大杠杆
    LeverMax float64 `json:"LeverMax,omitempty"`
    // 有效MMR
    MMR float64 `json:"MMR,omitempty"`
    // 有效MIR
    MIR float64 `json:"MIR,omitempty"`
    // 有Gift的时候
    PNLGISO float64 `json:"PNLGISO,omitempty"`
    // 计算值：价值,仓位现时的名义价值，受到标记价格价格的影响
    Val float64 `json:"Val,omitempty"`
    // 保证金，被仓位使用并锁定的保证金。
    MMnF float64 `json:"MMnF,omitempty"`
    MI float64 `json:"MI,omitempty"`
    // 计算值：未实现盈亏 PNL==  Profit And Loss
    UPNL float64 `json:"UPNL,omitempty"`
    // 计算值: 强平价格 亏光当前保证金的 (如果是多仓，并且标记价格低于PrzLiq,则会被强制平仓。/如果是空仓,并缺标记价格高于PrzLiq，则会被强制平仓
    PrzLiq float64 `json:"PrzLiq,omitempty"`
    // 计算值: 破产价格 BandRuptcy
    PrzBr float64 `json:"PrzBr,omitempty"`
    // 预估的平仓费
    FeeEst float64 `json:"FeeEst,omitempty"`
    // //////////////////////////////////////////////////////////////////////////
    // 下面会因为结算操作而变更。每次结算的时候，当前的未实现盈亏，将变成累加到已实现盈亏后，未实现盈亏清0
    ROE float64 `json:"ROE,omitempty"`
    // ADLIdx, 这个是用来排序ADL的
    ADLIdx float64 `json:"ADLIdx,omitempty"`
    // ADL红绿灯
    ADLLight int32 `json:"ADLLight,omitempty"`
}

// **用户钱包（合约和币币）**
type Wlt struct {
    // 投资者帐号
    UId string `json:"UId,omitempty"`
    // /Account Id
    AId string `json:"AId,omitempty"`
    // 货币类型
    Coin string `json:"Coin,omitempty"`
    // 钱包索引
    WId string `json:"WId,omitempty"`
    // 入金金额
    Depo gaea_Num.Flt `json:"Depo"`
    // 出金金额
    WDrw gaea_Num.Flt `json:"WDrw"`
    // 已实现盈亏
    PNL gaea_Num.Flt `json:"PNL"`
    // 冻结金额
    Frz gaea_Num.Flt `json:"Frz"`
    // ///////////////////////////////////////////////////////////////////////
    // 下面是统计值
    // 未实现盈亏：根据持仓情况、标记价格 刷新， 统计值
    UPNL float64 `json:"UPNL,omitempty"`
    // //////////////////////////////////////////
    // 委托保证金 = 计算自已有委单 + 平仓佣金 + 开仓佣金 Mgn Initial
    MI float64 `json:"MI,omitempty"`
    // 仓位保证金 + 平仓佣金 Mgn Maintaince
    MM float64 `json:"MM,omitempty"`
    // 风险度 // Risk Degree.
    RD float64 `json:"RD,omitempty"`
    // 可取余额 . 定时刷新。
    Wdrawable float64 `json:"Wdrawable,omitempty"`
    // 现货交易出入金
    Spot gaea_Num.Flt `json:"Spot,omitempty"`
    // 赠送金额 不允许取出。
    Gift gaea_Num.Flt `json:"Gift,omitempty"`
    // Gift不为0的时候
    PNLG gaea_Num.Flt `json:"PNLG,omitempty"`
    // 账户状态
    Status WltStatus `json:"Status,omitempty"`
}

// 钱包日志
type WltLog struct {
    UId string `json:"UId,omitempty"x`
    AId string `json:"AId,omitempty"`
    Seq string `json:"Seq,omitempty"`
    // 货币类型
    Coin string `json:"Coin,omitempty"`
    // 投资者帐号
    WId string       `json:"WId,omitempty"`
    Qty gaea_Num.Flt `json:"Qty"`
    Fee gaea_Num.Flt `json:"Fee"`
    // 货币地址(假设是出金，则是地址)
    Peer string `json:"Peer,omitempty"`
    // 在进行完本次操作后，钱包的CalcWltBal函数的返回值。请注意，在合约交易中，当逐仓保证金变化的时候，本字段不会有对应的记录。
    WalBal gaea_Num.Flt `json:"WalBal"`
    // 时间
    At int64 `json:"At,omitempty"`
    // 类型
    Op WltOp `json:"Op,omitempty"`
    // 来源
    Via OrderVia `json:"Via,omitempty"`
    // Info
    Info string `json:"Info,omitempty"`
    // 错误代码
    ErrCode ErrorCode `json:"ErrCode,omitempty"`
    // 错误文本
    ErrTxt    string      `json:"ErrTxt,omitempty"`
    Stat      OrderStatus `json:"Stat,omitempty"`
}

type TrdRec struct {
    // 投资者帐号
    UId             string       `json:"UId,omitempty"`
    AId             string       `json:"AId,omitempty"`
    Sym             string       `json:"Sym,omitempty"`
    WId             string       `json:"WId,omitempty"`
    MatchId         string       `json:"MatchId,omitempty"`
    OrdId           string       `json:"OrdId,omitempty"`
    Sz              gaea_Num.Flt `json:"Sz"`
    Prz             gaea_Num.Flt `json:"Prz"`
    Fee             gaea_Num.Flt `json:"Fee"`
    FeeCoin         string       `json:"FeeCoin,omitempty"`
    At              int64        `json:"At,omitempty"`
    Via             OrderVia     `json:"Via,omitempty"`
    GrossVal        float64      `json:"GrossVal,omitempty"`
    HomeNotional    float64      `json:"HomeNotional,omitempty"`
    ForeignNotional float64      `json:"foreignNotional,omitempty"`
    // 下面的数据，来自Trdsum
    // 平均买入价
    BAvg float64 `json:"BAvg,omitempty"`
    // 计算平均值的买入量
    NBid float64 `json:"NBid,omitempty"`
    // 平均卖出价
    AAvg float64 `json:"AAvg,omitempty"`
    // 计算平均值的卖出量
    NAsk float64 `json:"NAsk,omitempty"`
    // 统计周期内买入量
    SzBid float64 `json:"SzBid,omitempty"`
    // 统计周期内卖出量
    SzAsk float64 `json:"SzAsk,omitempty"`
    // 统计周期内买入次数
    NumBid uint64 `json:"NumBid,omitempty"`
    // 统计周期内卖出次数
    NumAsk uint64 `json:"NumAsk,omitempty"`
    // 算力等级
    MPL int64 `json:"MPL,omitempty"`
    // 买入算力 Mine Power for Bid
    MPB float64 `json:"MPB,omitempty"`
    // 卖出算力 Mine Power for Ask
    MPA float64 `json:"MPA,omitempty"`
    // 算力相关量. 可能并不会等于 Sz
    MPS float64 `json:"MPS,omitempty"`
    // 扩展字段
    Ext string `json:"Ext,omitempty"`
}



//
// 委托的状态
type OrderStatus int32

const (
	// 未指定
	OrderStatus_OS_Invalid OrderStatus = 0
	// 正在排队
	OrderStatus_Queueing OrderStatus = 1
	// 有效
	OrderStatus_Matching OrderStatus = 2
	// 提交失败
	OrderStatus_PostFail OrderStatus = 3
	// 已执行
	OrderStatus_Executed OrderStatus = 4
)


type OfferType int32

const (
	OfferType_OT_Invalid OfferType = 0
	// 限价委单
	OfferType_Limit OfferType = 1
	// 市价委单,匹配后转限价
	OfferType_Market OfferType = 2
	// 限价止损/盈利
	OfferType_StopLimit OfferType = 3
	// 市价止损/盈利
	OfferType_StopMarket OfferType = 4
	// 追踪 限价
	OfferType_TraceLimit OfferType = 5
	// 追踪 市价
	OfferType_TraceMarket OfferType = 6
)


//
// 条件委托触发的判据
type StopBy int32

const (
	// 标记价格
	StopBy_PriceMark StopBy = 0
	// 最新成交
	StopBy_PriceLatest StopBy = 1
	// 指数价格
	StopBy_PriceIndex StopBy = 2
)


//
// 交易指令的标志
type OrdFlag int32

const (
	// 占位，无意义
	OrdFlag_OF_INVALID OrdFlag = 0
	// 如果委托会立即成交，则不发送此委托
	OrdFlag_POSTONLY OrdFlag = 1
	// 如果委托会导致增加仓位，则不发送此委托
	OrdFlag_REDUCEONLY OrdFlag = 2
	// 触发后平仓 TODO 目前未实现
	// 	CLOSEONTRIGGER 	= 4;
	// 条件指定为 如果价格大于StopBy
	OrdFlag_IF_GREATERTHAN OrdFlag = 8
	// 条件指定为 如果价格低于StopBy
	OrdFlag_IF_LESSTHAN OrdFlag = 16
	// 行情追踪委托的激活状态
	OrdFlag_TRACE_ACTIVE OrdFlag = 32
	// 行情追踪委托的触发状态
	OrdFlag_TRACE_FIRE OrdFlag = 64
	// 设定此标志以跟踪最大值的回调。不设定此标志以跟踪最小值的回调
	OrdFlag_TRACE_AT_MAX OrdFlag = 128
)


//
//
type TimeInForce int32

const (
	// 一直有效
	TimeInForce_GoodTillCancel TimeInForce = 0
	// 部分成交后剩余委托取消
	TimeInForce_ImmediateOrCancel TimeInForce = 1
	// 部分成交后剩余委托取消
	TimeInForce_FillAndKill TimeInForce = 1
	// 如果不能全部成交则取消委托			全部成交或者全部撤销
	TimeInForce_FillOrKill TimeInForce = 2
)



//
//
type TradeClass int32

const (
	TradeClass_TC_INVALID TradeClass = 0
	// Spot Trading 现货交易
	TradeClass_TC_SPOT TradeClass = 1
	// Future Trading 期货交易
	TradeClass_TC_FUTURE TradeClass = 2
	// 永续
	TradeClass_TC_PERPETUAL TradeClass = 3
)

```


## 相关术语
| 名称 | 描述 |
| :------: | :------ |
|UId | 是用户的系统内部唯一编号, 例: UId:123456|
|AId | 子账户ID, AId是在UId的后面添加两位数字生成, 01为合约ID, 02为现货ID, 例: UId:123456加01则为AId:12345601|
|rid | 用户发送请求的唯一编号，由于websocket是异步通讯，用户需要通过匹配收到消息的rid和自己发送的rid来匹配操作和应答,值必须为字符串类型|



## 错误码定义

| ErrCode| ErrTxt | 描述 |
|:------:|:------|:------|
| 0       |  NORERROR| 没有错误  |
| 1       |  GENERAL | 数据错误 |
| 2       |  DATA    | 数据错误 |
| 3       |  NOT_IMPLEMENTED     | 服务器未实现 |
| 4       |  NO_MARGIN     | 保证金不足 |
| 5       |  FATAL     | 致命错误 |
| 6       |  NOT_FOUND     | 未找到 |
| 7       |  UNKNOWN_DIR     | 未知的委托方向 |
| 8       |  INVALID_CODE     | 操作码错误 |
| 9       |  EXISTS     | 已存在 |
| 10      |  NOT_FOUND_ORD     | 没有找到订单号 |
| 11      |  PRZ_INVALID     | 价格错误 |
| 12      |  EXPIRED     | 已过期 |
| 13      |  NOT_SUFFICIENT     | 资金不足 |
| 14      |  WILLFILL     | 对于PostOnly，本委托会成交 |
| 15      |  EXECUTE_FAIL     | 对FillOrKill委托，这表示执行撮合失败 |
| 16      |  --UNUSED--     | --UNUSED-- |
| 17      |  ORDQTY_TOO_BIG_TOO_SMALL     | 委托价值太小 |
| 18      |  EXCEED_LIMIT_PRZ_QTY     | 价格或者数量超出限制 |
| 19      |  DENYOPEN_BY_POS     | 仓位超出限制 |
| 20      |  DENYOPEN_BY_RD     | 禁止开仓 |
| 21      |  TRADE_STOPED     |  交易暂停 |
| 22      |  EXCEED_PRZ_LIQ     | 超过强平价格 |
| 23      |  TOO_MANY_ORDER     | 太多的委托 |
| 24      |  DENYOPEN_BY_TIME     | 超出开仓时间限制 |
| 25      |  MD5_INVALID     | MD5签名验证错误 |
| 26      |  RATELIMIT     | 限速,每秒50次API调用 |
| 27      |  USER_CANCELED     | 用户撤销 |
| 28      |  NOT_FOUND_WLT     | 无法找到钱包 |
| 29      |  NOT_FOUND_MKT     | 未找到交易对 |
| 30      |  EXCEED_MAXORDVAL     | 超过最大委托价值 |
| 31      |  WILL_LIQUIDATE     | 将导致爆仓、强平 |
| 32      |  NOT_IN_TRADE_PERIOD     | 非交易时间 |
| 33      |  EXCEED_RAISE_FALL_R     | 超过涨跌停价格限制 |
| 34      |  PRZ_TOO_LOW     | 超出最小价格限制 |
| 35      |  EXCEED_TRADE_VOL     | 超出交易量限制 |
| 36      |  EXCEED_TRADE_COUNT     | 超出交易次数限制 |
| 37      |  EXCEED_ASK_BID_PRZ_RATE     | 委托价格 超过盘口最新价格偏离 |
| 64      |  NO_DEFAULT_RISKLIMIT     | 没有指定风险限额 |

