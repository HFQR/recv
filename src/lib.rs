pub mod macros;

use std::ops::{Add, Div, Mul, Sub};

pub fn code_to_u64(str: &'static str) -> u64 {
    let mut buf = [0; 8];
    for (idx, char) in str.as_bytes().iter().enumerate() {
        buf[idx] = *char;
    }
    u64::from_le_bytes(buf)
}

pub trait TickDataStructure<T>
where
    T: Add + Mul + Div + Sub + PartialEq + PartialOrd,
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

    // 毫秒数
    fn ms(&self) -> u16;

    // 合约代码名称 也许是其他的数字 用于后期转换判断
    fn code(&self) -> u64;

    fn ask_volume_all(&self) -> T;

    fn bid_volume_all(&self) -> T;

    fn buy_volume(&self) -> T;

    fn sell_volume(&self) -> T;
}

pub trait Message<F, T>
where
    F: Add + Mul + Div + Sub + PartialEq + PartialOrd,
    T: Default,
{
    fn on_tick(&mut self, tick: impl TickDataStructure<F>);

    fn on_order(&mut self, call: T);
}
