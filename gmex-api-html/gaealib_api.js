/**
 * A JavaScript implementation of the GAEA-LIB API. 
 */

function gaea(options) {
    this.rid = 0;
    this.msgMap = {};  // rid - msg
    this.listeners = {};

    this.api_secret = ""; // 用于消息签名
    return this;
}

gaea.prototype.init = function (options, cb) {
    if (typeof options !== 'object') return;
    var self = this;

    self.api_secret = options.api_secret || ""
    self.timeout = options.timeout || 20000;
    self.notimeout = options.notimeout || 10000;
    self.activetm = 0;
    self.msgMap = {};
    if (self.tmrClean) clearInterval(self.tmrClean);
    if (self.tmr) clearInterval(self.tmr);
    self.tmrClean = setInterval(function () {
        var tm = Date.now();
        for (var rid in self.msgMap) {
            var m = self.msgMap[rid];
            if (m.tm + self.timeout < tm) {
                if (typeof (m.cb) == 'function') {
                    console.log(`[timeout]<< ${JSONStringify(m)}`)
                    // console.log(m)
                    m.cb({ code: -9999, rid: rid, data: 'timeout' })
                }
                delete self.msgMap[rid];
            }
        }
    }, 1000);

    if (typeof cb == 'function') self.listeners['open'] = cb;

    var onPush = function (route, msg) {
        if (typeof (self.listeners[route]) == 'function') self.listeners[route](msg);
    }

    self.close();
    var ws = new WebSocket(options.ws_url);
    ws.binaryType = "arraybuffer"; // 默认的"blob"在解压时不方便用这里直接改掉
    ws.onopen = function (evt) {
        self.connected = true;
        self.activetm = Date.now();
        self.tmr = setInterval(function () {
            if (self.activetm + self.notimeout > Date.now()) return;
            if (self.activetm + self.timeout < Date.now()) {
                self.close();
                return;
            }
            if (self.connected) {
                self.request('Time', Date.now(), (ret) => {
                    //console.log('Server Time:', ret.data)
                    self.activetm = Date.now();
                })
            }
        }, self.notimeout / 2);
        onPush('open')
    }
    ws.onmessage = function (evt) {
        self.activetm = Date.now();
        var msg = {}
        try {
            if (typeof(evt.data) == 'string') {
                msg = JSON.parse(evt.data);
            } else {
                msg = JSON.parse(pako.inflateRaw(evt.data, { to: 'string' }));
            }
        } catch (e) {
            console.log(`[onmessage.decode.err]<< `, e, evt)
            return;
        }

        var rid = msg.rid;
        var m = self.msgMap[rid];
        if (rid && m && typeof (m.cb) == 'function') {
            m.cb(msg);
        } else if (msg.subj && typeof (self.listeners[msg.subj]) == 'function') {
            self.listeners[msg.subj](msg.data);
        }
        delete self.msgMap[rid];
    };
    ws.onclose = function (evt) {
        self.connected = false;
        clearInterval(self.tmr);
        onPush('close')
    };
    ws.onerror = function (evt) {
        onPush('error')
    };

    self.ws = ws;
}

gaea.prototype.on = function (route, cb) {
    this.listeners[route] = cb;
}
gaea.prototype.remove = function (route) {
    delete this.listeners[route];
}
gaea.prototype.close = function () {
    if (this.connected)
        this.ws.close();
    if (this.ws) {
        this.ws.onclose = function () { }
        this.ws.close();
        delete this.ws;
    }
    this.connected = false;
}

gaea.prototype.is_connected = function() {
    return this.ws && this.connected
}

gaea.prototype.request = function (route, pmsg, cb) {
    if (arguments.length !== 3 || typeof route != 'string' || typeof (cb) != 'function' || !this.ws) {
        console.log('params error', arguments.length !== 3, typeof route != 'string', typeof (cb) != 'function', this.ws)
        return;
    }
    var msg = {}
    msg.req = route;
    msg.rid = String(this.rid++);
    msg.expires = Date.now() + 3500;
    msg.args = pmsg;
    
    if ( this.api_secret && this.api_secret.length > 1 ) {
        // 消息签名，请注意参数的顺序.
        txtbody = `${msg.req}${msg.rid}${JSON.stringify(msg.args)}${msg.expires}${this.api_secret}`
        msg.signature = hex_md5(txtbody)
        //console.log('msgbody: ', txtbody, msg.signature)
    }

    this.msgMap[msg.rid] = { tm: Date.now(), cb: cb };
    try {
        if (this.connected)
            this.ws.send(JSON.stringify(msg));
    } catch (e) { }
}


// export default {
//     gaea
// }
module = gaea;