use rusty_money::{Money, iso};

#[derive(Debug, Clone)]
pub struct Price(Money<'static, iso::Currency>);
