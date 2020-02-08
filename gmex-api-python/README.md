# gmex-api-python

当前代码完成了 websocket 的封装，Model 类还没找到合适的py解决方案，暂时手写
了Order和Postion的封装，后继看有啥好的方法重写，暂时建议大家直接用dict参考
文档看各个字段的意思吧。
通常各自使用的技术栈都有各种model方案，请大家推荐好的给我。

REST 部分暂时没有示例，其都是简单的http post请求，参考js或dotnet的实现应该
很容易自己搞定。

有啥好的建议和想法，请email我<hexiaoyuan@126.com>

# 开发测试相关

开发环境:
brew install python pipenv

进入当前目录:
cd ~/your-gmex-api/gmex-api-python

设置虚拟环境:
pipenv --three
pipenv install --dev
pipenv update

开启虚拟环境:
pipenv shell

创建用户配置文件 user_cfg.json, 内容如下：
```
{
    "prod": {
        "trd_ws_url": "wss://api-trade.gmex.io/v1/trade",
        "mkt_ws_url": "wss://api-market.gmex.io/v1/market",
        "user_name": "youremail@google.com",
        "api_key": "<your-api-key-string>",
        "api_secret": "<your-api-secret-string>"
    }
}
```

运行演示程序:
python main.py

