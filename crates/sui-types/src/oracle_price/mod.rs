use std::string::FromUtf8Error;
use crate::collection_types::VecMap;
use jsonrpsee::core::Serialize;
use serde::Deserialize;
use crate::base_types::ObjectID;
use crate::error::SuiError;
use crate::id::UID;
use crate::storage::ObjectStore;

const BUSD_COIN_TYPE: &str = "00000000000000000000000000000000000000000000000000000000000000c8::busd::BUSD";
#[derive(Debug, Hash, Eq, Serialize, Deserialize, Clone, PartialEq)]
pub struct PriceIdentifier {
    pub coin_type_a: Vec<u8>,
    pub coin_type_b: Vec<u8>,
}

impl PriceIdentifier {
    pub fn new(coin_type_a: &[u8], coin_type_b: &[u8]) -> Self {
        Self {
            coin_type_a: coin_type_a.to_vec(),
            coin_type_b: coin_type_b.to_vec(),
        }
    }

    pub fn coin_type_a_string(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.coin_type_a.clone())
    }

    pub fn coin_type_b_string(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.coin_type_b.clone())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OraclePrice {
    pub id: UID,
    pub name: Vec<u8>,
    pub value: VecMap<PriceIdentifier, u64>,
}

impl OraclePrice {
    pub fn get_price(&self, id: &PriceIdentifier) -> Option<u64> {
        for e in self.value.contents.iter() {
            if e.key == *id {
               return Some(e.value)
            }
        }

        None
    }

    pub fn to_exchange_rate_against_busd(&self) -> (Vec<String>, Vec<u64>) {
        let mut data: Vec<(String, u64)> = vec![];
        for e in self.value.contents.iter() {
            let coin_type_a = String::from_utf8(e.key.coin_type_a.clone()).unwrap();
            let coin_type_b = String::from_utf8(e.key.coin_type_b.clone()).unwrap();

            if coin_type_b == BUSD_COIN_TYPE {
                data.push((coin_type_a, e.value));
            }
        }

        data.sort();

        let mut stable_coin_type: Vec<String> = Vec::with_capacity(data.len());
        let mut stable_coin_rate_against_busd: Vec<u64> = Vec::with_capacity(data.len());
        for v in data {
            stable_coin_type.push(v.0);
            stable_coin_rate_against_busd.push(v.1);
        }

        (stable_coin_type, stable_coin_rate_against_busd)
    }
}

pub fn get_oracle_price_by_id(
    object_store: &dyn ObjectStore,
    id: ObjectID,
) -> anyhow::Result<OraclePrice, SuiError> {
    let wrapper = object_store
        .get_object(&id)
        .ok_or_else(|| {
            SuiError::OraclePriceReadError(format!("OraclePrice object({}) not found",
                                                   &id.to_string()).to_owned())
        })?;
    let move_object = wrapper.data.try_as_move().ok_or_else(|| {
        SuiError::OraclePriceReadError(
            format!("OraclePrice object({}) must be a Move object",
                    &id.to_string()).to_owned(),
        )
    })?;
    let result = bcs::from_bytes::<OraclePrice>(move_object.contents())
        .map_err(|err| SuiError::OraclePriceReadError(err.to_string()))?;
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_oracle_price() {
        let str = r#"a364350796396174d26f7c085dab053996412cd9ee998604ecee3ba33a6a8b780d66785f70726963655f64617461014c303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303063383a3a626575723a3a424555524c303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303063383a3a627573643a3a42555344a086010000000000"#;
        let data = hex::decode(str).unwrap();
        let result = bcs::from_bytes::<OraclePrice>(&data).unwrap();
        println!("{:?}", result);

        // vec to string
        let name = String::from_utf8(result.name.clone()).unwrap();
        println!("{:?}", name);
        assert_eq!(name, "fx_price_data");
        result.value.contents.iter().for_each(|e| {
            // 00000000000000000000000000000000000000000000000000000000000000c8::beur::BEUR
            let coin_type_a = e.key.coin_type_a_string().unwrap();
            // 00000000000000000000000000000000000000000000000000000000000000c8::busd::BUSD
            let coin_type_b = e.key.coin_type_b_string().unwrap();
            println!("{}/{}", coin_type_a, coin_type_b);
            println!("{}", e.value);
        });
        assert_eq!(
            result.get_price(&PriceIdentifier::new(
                b"00000000000000000000000000000000000000000000000000000000000000c8::beur::BEUR",
                b"00000000000000000000000000000000000000000000000000000000000000c8::busd::BUSD")).unwrap(),
            100000,
        );

        let (stable_coin_type, stable_coin_rate_against_busd) = result.to_exchange_rate_against_busd();

        for (i, k)  in stable_coin_type.iter().enumerate()  {
            if k == "00000000000000000000000000000000000000000000000000000000000000c8::beur::BEUR" {
                assert_eq!(
                    stable_coin_rate_against_busd.get(i),
                    Some(&100000),
                )
            }
        }        
    }
}