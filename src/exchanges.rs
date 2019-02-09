pub mod binance;
// pub mod bittrex;
// pub mod bitfinex;
// pub mod kucoin;

use serde_derive::Deserialize;

use crate::error::*;
use crate::models::*;

pub trait ExchangeAPI {
    fn display(&self) -> String;
    fn btc_symbol(&self) -> String;
    fn usd_symbol(&self) -> String;
    fn base_pairs(&self) -> Vec<String>;
    fn btc_price(&self) -> Result<Pair, TrailerError>;
    fn balances(&self) -> Result<Vec<Asset>, TrailerError>;
    fn pair(&self, pair: &str) -> Result<Pair, TrailerError>;
    fn all_pairs(&self) -> Result<Vec<Pair>, TrailerError>;
    fn limit_buy(&self, symbol: &str, amount: f64, price: f64) -> Result<(), TrailerError>;
    fn limit_sell(&self, symbol: &str, amount: f64, price: f64) -> Result<(), TrailerError>;
    fn stop_loss(
        &self,
        symbol: &str,
        amount: f64,
        stop_price: f64,
        limit_price: f64,
    ) -> Result<(), TrailerError>;
    fn open_orders(&self) -> Result<Vec<Order>, TrailerError>;
    fn past_orders(&self) -> Result<Vec<Order>, TrailerError>;

    fn trades_for(&self, symbol: &str) -> Result<Vec<Order>, TrailerError>;
    fn _trades_for(&self, symbol: &str) -> Result<Vec<Trade>, TrailerError>;

    fn chart_data(&self, symbol: &str, interval: &str) -> Result<Vec<Candlestick>, TrailerError>;

    fn pair_format(&self, pair: Pair) -> String;
    fn string_to_pair(&self, pair: String, price: f64) -> Pair;

    fn btc_pair(&self, pairs: Vec<Pair>) -> Option<Pair> {
        find_pair_by_symbol_and_base(&self.btc_symbol(), &self.usd_symbol(), pairs)
    }

    fn symbol_and_base_to_pair_format(&self, symbol: &str, base: &str) -> String;

    fn usd_pair(&self, pairs: Vec<Pair>) -> Option<Pair> {
        find_pair_by_symbol_and_base(&self.usd_symbol(), &self.btc_symbol(), pairs)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub enum Exchange {
    Unknown,
    Bittrex,
    Binance,
    Kucoin,
}

impl std::str::FromStr for Exchange {
    type Err = ();

    fn from_str(s: &str) -> Result<Exchange, ()> {
        match s {
            "unknown" => Ok(Exchange::Unknown),
            "-" => Ok(Exchange::Unknown),
            "bittrex" => Ok(Exchange::Bittrex),
            "binance" => Ok(Exchange::Binance),
            "kucoin" => Ok(Exchange::Binance),
            _ => Err(()),
        }
    }
}

use std::string::ToString;
impl ToString for Exchange {
    fn to_string(&self) -> String {
        match self {
            &Exchange::Bittrex => "bittrex".into(),
            &Exchange::Binance => "binance".into(),
            &Exchange::Kucoin => "kucoin".into(),
            _ => "-".into(),
        }
    }
}
