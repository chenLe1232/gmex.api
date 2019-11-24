NOTE: 运行examples前请创建 `.env` 文件.

$ cat .env

GMEX_HTTP_URL_MARKET=https://api-market.gmex.io/v1/rest
GMEX_HTTP_URL_TRADE=https://api-trade.gmex.io/v1/rest

GMEX_WS_URL_MARKET=wss://api-market.gmex.io/v1/market
GMEX_WS_URL_TRADE=wss://api-trade.gmex.io/v1/trade

GMEX_USER_NAME=<your@email.com>
GMEX_API_KEY=xxxxxxxxxxxxxxxxxxxxxxxxxxx
GMEX_API_SECRET=yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy

跑例子试试：
cd examples/http_client; cargo run
cd examples/websocket_client; cargo run


*NOTE*：
当前代码使用 rust_decimal 的大数，但是其默认的 json 序列化是 string，但是服务端过来
的大数还是数字形式，是否存在反序列化丢失精度问题，有待测试。
