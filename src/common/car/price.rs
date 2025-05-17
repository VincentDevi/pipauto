use rust_decimal::{Decimal, prelude::FromPrimitive};
use rusty_money::{FormattableCurrency, Money, iso};
use serde::{Deserialize, Serialize, ser::SerializeStruct};

#[derive(Debug, Clone)]
pub struct Price(Money<'static, iso::Currency>);

impl Serialize for Price {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Price", 2)?;
        state.serialize_field("amount", &self.0.amount().to_string())?;
        state.serialize_field("currency", &self.0.currency().code())?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Price {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct PriceData {
            amount: String,
            currency: String,
        }

        let data = PriceData::deserialize(deserializer)?;

        let amount = data
            .amount
            .parse::<Decimal>()
            .map_err(|e| serde::de::Error::custom(format!("Failed to parse amount: {}", e)))?;

        let currency = match data.currency.as_str() {
            "EUR" => iso::EUR,
            "USD" => iso::USD,
            "GBP" => iso::GBP,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Unsupported currency: {}",
                    data.currency
                )));
            }
        };

        let money = Money::from_decimal(amount, currency);
        Ok(Price(money))
    }
}

impl TryFrom<f64> for Price {
    type Error = String;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Ok(Self(Money::from_decimal(
            Decimal::from_f64(value).ok_or("cannot parse f64 to Price")?,
            iso::EUR,
        )))
    }
}
