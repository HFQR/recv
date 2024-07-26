pub mod macros;

use core::fmt;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use std::ops::{Add, Div, Mul, Sub};

pub fn code_to_u64(str: &'static str) -> u64 {
    let mut buf = [0; 8];
    for (idx, char) in str.as_bytes().iter().enumerate() {
        buf[idx] = *char;
    }
    u64::from_le_bytes(buf)
}

/// timestamp: Second
/// ms: 0-1000 ms
#[derive(Debug)]
pub struct Time {
    pub timestamp: u64,
    pub ms: u16,
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let timestamp = self.timestamp + 8 * 3600;
        let seconds = timestamp % (24 * 3600);
        let hour = seconds / 3600;
        let minute = (seconds - hour * 3600) / 60;
        let second = seconds % 60;
        write!(
            f,
            "{:0>2}:{:0>2}:{:0>2}.{:0>3}",
            hour, minute, second, self.ms
        )
    }
}

pub trait TickDataStructure<T>
where
    T: Add + Mul + Div + Sub + PartialEq + PartialOrd + Into<f64>,
{
    // 成交均价

    fn last_price(&self) -> f64;

    //成交量

    fn volume(&self) -> T;

    //持仓量

    fn open_interest(&self) -> T;

    // 五档买方挂单价格

    fn bid_price(&self, index: usize) -> f64;

    // 五档卖方挂单价格
    fn ask_price(&self, index: usize) -> f64;

    // 五档买挂单量
    fn bid_volume(&self, index: usize) -> T;

    // 五档卖挂单量
    fn ask_volume(&self, index: usize) -> T;

    // 中间价
    fn mid_price(&self) -> f64;
    // 成交金额
    fn turnover(&self) -> f64;

    //根据当天其实的时间计算出时分秒
    fn hms(&self, base_time: u32) -> (u32, u32, u32);

    // 当天 小时 + 分钟 + 秒数 时间戳
    fn timestamp(&self, base_time: u64) -> u64;

    // 时间戳
    fn snap_time(&self) -> u64;

    // 可以被忽略 仅用于本地回测
    fn signal(&self) -> f64 {
        0.0
    }

    fn datetime(&self) -> String {
        "".to_string()
    }

    // 毫秒数
    fn ms(&self) -> u16;
    // 合约代码名称 也许是其他的数字 用于后期转换判断
    fn code(&self) -> u64;

    fn ask_volume_all(&self) -> T;

    fn bid_volume_all(&self) -> T;

    fn buy_volume(&self) -> T;

    fn sell_volume(&self) -> T;

    fn time(&self) -> Time {
        Time {
            timestamp: self.snap_time(),
            ms: self.ms(),
        }
    }

    fn columns(&self) -> [&'static str; 29] {
        [
            "last_price",
            "mid_price",
            "volume",
            "turnover",
            "buy_volume",
            "sell_volume",
            "open_interest",
            "snap_time",
            "ms",
            "ask_price_1",
            "ask_price_2",
            "ask_price_3",
            "ask_price_4",
            "ask_price_5",
            "ask_volume_1",
            "ask_volume_2",
            "ask_volume_3",
            "ask_volume_4",
            "ask_volume_5",
            "bid_price_1",
            "bid_price_2",
            "bid_price_3",
            "bid_price_4",
            "bid_price_5",
            "bid_volume_1",
            "bid_volume_2",
            "bid_volume_3",
            "bid_volume_4",
            "bid_volume_5",
        ]
    }

    fn display(&self) -> [f64; 29] {
        [
            self.last_price(),
            self.mid_price(),
            self.volume().into(),
            self.turnover(),
            self.buy_volume().into(),
            self.sell_volume().into(),
            self.open_interest().into(),
            self.snap_time() as f64,
            self.ms() as f64,
            self.ask_price(0),
            self.ask_price(1),
            self.ask_price(2),
            self.ask_price(3),
            self.ask_price(4),
            self.ask_volume(0).into(),
            self.ask_volume(1).into(),
            self.ask_volume(2).into(),
            self.ask_volume(3).into(),
            self.ask_volume(4).into(),
            self.bid_price(0),
            self.bid_price(1),
            self.bid_price(2),
            self.bid_price(3),
            self.bid_price(4),
            self.bid_volume(0).into(),
            self.bid_volume(1).into(),
            self.bid_volume(2).into(),
            self.bid_volume(3).into(),
            self.bid_volume(4).into(),
        ]
    }
}

#[derive(Eq, PartialOrd, PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
#[repr(u8)]
pub enum Direction {
    Long,
    Short,
}

#[derive(Eq, PartialOrd, PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
#[repr(u8)]
pub enum Offset {
    Open,
    Close,
    CloseToday,
    CloseYesterday,
}

#[derive(Eq, PartialOrd, PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
#[repr(u8)]
pub enum Exchange {
    SHFE,    // 上海期货交易所
    CFFEX,   // 中国金融交易所
    INE,     // 能源交易所
    BINANCE, // 币安交易所
    CZCE,    // 中金所
    DCE,     // 大商所
}

#[derive(Eq, PartialOrd, PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
#[repr(u8)]
pub enum OrderType {
    Limit, // 限价订单
    Fak,
    Fok,
    Market, // 市价单
}

#[derive(Eq, PartialOrd, PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
#[repr(u8)]
pub enum Status {
    Submitting,
    NotTraded,
    Traded,
    Cancel,
}

pub trait Order<'a> {
    fn price(&self) -> f64 {
        unimplemented!()
    }

    //手数
    fn volume(&self) -> f64 {
        unimplemented!()
    }

    // 方向
    fn direction(&self) -> Direction {
        unimplemented!()
    }

    // 时间
    fn datetime(&self) -> String {
        unimplemented!()
    }

    //执行类型
    fn offset(&self) -> Offset {
        unimplemented!()
    }

    fn status(&self) -> Status {
        unimplemented!()
    }

    fn order_type(&self) -> OrderType {
        unimplemented!()
    }

    fn token(&self) -> u32 {
        unimplemented!()
    }

    fn order_id(&self) -> u64 {
        unimplemented!()
    }

    // 此函数用于数据变为u8字节流
    fn dummy(&self) -> &'a [u8] {
        unimplemented!()
    }
}
