pub mod macros;

use core::fmt;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt::{Debug, Display};
use std::time::Duration;

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum Direction {
    LONG = 1,
    SHORT = 0,
}

impl Debug for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::LONG => write!(f, "LONG"),
            Self::SHORT => write!(f, "SHORT"),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::LONG => write!(f, "LONG"),
            Self::SHORT => write!(f, "SHORT"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum Offset {
    OPEN = 1,
    CLOSE = 2,
    CLOSETODAY = 3,
    CLOSEYESTERDAY = 4,
}

impl Debug for Offset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::OPEN => write!(f, "OPEN"),
            Self::CLOSE => write!(f, "CLOSE"),
            Self::CLOSETODAY => write!(f, "CLOSETODAY"),
            Self::CLOSEYESTERDAY => write!(f, "CLOSEYESTERDAY"),
        }
    }
}

impl Display for Offset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::OPEN => write!(f, "OPEN"),
            Self::CLOSE => write!(f, "CLOSE"),
            Self::CLOSETODAY => write!(f, "CLOSETODAY"),
            Self::CLOSEYESTERDAY => write!(f, "CLOSEYESTERDAY"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum Status {
    ERROR = 0,
    SUBMITTING = 1,
    NOTTRADED = 2,
    ALLTRADED = 3,
    CANCELLED = 4,
    CANCELFAILED = 5,
}

impl Debug for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::ERROR => write!(f, "ERROR"),
            Self::SUBMITTING => write!(f, "SUBMITTING"),
            Self::NOTTRADED => write!(f, "NOTTRADED"),
            Self::ALLTRADED => write!(f, "ALLTRADED"),
            Self::CANCELLED => write!(f, "CANCELLED"),
            Self::CANCELFAILED => write!(f, "CANCELFAILED"),
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::ERROR => write!(f, "ERROR"),
            Self::SUBMITTING => write!(f, "SUBMITTING"),
            Self::NOTTRADED => write!(f, "NOTTRADED"),
            Self::ALLTRADED => write!(f, "ALLTRADED"),
            Self::CANCELLED => write!(f, "CANCELLED"),
            Self::CANCELFAILED => write!(f, "CANCELFAILED"),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum OrderType {
    LIMIT = 0,
    MARKET = 1,
    STOP = 2,
    FAK = 3,
    FOK = 4,
}

impl Debug for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::LIMIT => write!(f, "LIMIT"),
            Self::MARKET => write!(f, "MARKET"),
            Self::STOP => write!(f, "STOP"),
            Self::FAK => write!(f, "FAK"),
            Self::FOK => write!(f, "FOK"),
        }
    }
}

impl Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::LIMIT => write!(f, "LIMIT"),
            Self::MARKET => write!(f, "MARKET"),
            Self::STOP => write!(f, "STOP"),
            Self::FAK => write!(f, "FAK"),
            Self::FOK => write!(f, "FOK"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Exchange {
    SHFE = 0,
    CFFEX = 1,
    CZCE = 2,
    DCE = 3,
    INE = 4,
    COMEX = 5,
}

impl Default for Exchange {
    fn default() -> Self {
        Self::SHFE
    }
}

impl Debug for Exchange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::SHFE => write!(f, "SHFE"),
            Self::CFFEX => write!(f, "CFFEX"),
            Self::CZCE => write!(f, "CZCE"),
            Self::DCE => write!(f, "DCE"),
            Self::INE => write!(f, "INE"),
            Self::COMEX => write!(f, "COMEX"),
        }
    }
}

impl Display for Exchange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::SHFE => write!(f, "SHFE"),
            Self::CFFEX => write!(f, "CFFEX"),
            Self::CZCE => write!(f, "CZCE"),
            Self::DCE => write!(f, "DCE"),
            Self::INE => write!(f, "INE"),
            Self::COMEX => write!(f, "COMEX"),
        }
    }
}

/// Order Data
/// In here
#[derive(Clone, Debug, Copy)]
pub struct OrderData {
    pub symbol: u64,
    pub exchange: Exchange,
    pub token: u32,
    pub order_id: u64,
    pub trade_id: u64,
    pub order_type: OrderType,
    pub direction: Direction,
    pub offset: Offset,

    pub price: f64,
    pub volume: u32,
    pub status: Status,
    pub timestamp: Duration,
}

/// Order Request
#[derive(Clone, Debug, Copy)]
pub struct OrderRequest {
    pub symbol: &'static str,
    pub exchange: Exchange,
    pub direction: Direction,
    pub order_type: OrderType,
    pub volume: u32,
    pub price: f64,
    pub offset: Offset,
    pub reference: u32,
}

/// Cancel Request
#[derive(Clone, Debug, Copy)]
pub struct CancelRequest {
    pub order_id: u64,
}