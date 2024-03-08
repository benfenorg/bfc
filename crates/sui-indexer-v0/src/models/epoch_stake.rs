use diesel::{Insertable, Queryable};
use move_core_types::parser::parse_type_tag;
use sui_json_rpc_types::StakeCoin;
use sui_types::TypeTag;

use crate::schema::{epoch_stake_coins, epoch_stakes};

#[derive(Queryable, Insertable, Debug, Clone, Default)]
#[diesel(table_name = epoch_stakes)]
pub struct EpochStake {
    pub epoch: i64,
    pub total_stake: i64,
    pub total_reward: i64,
    pub accumulated_reward: i64,
    pub avg_exchange_rate: i64,
    pub apy: i64,
}

#[derive(Queryable, Insertable, Debug, Clone, Default)]
#[diesel(table_name = epoch_stake_coins)]
pub struct EpochStakeCoin {
    pub epoch: i64,
    pub coin_type: String,
    pub coin_balance: i64,
    pub bfc_value: i64,
}

impl From<EpochStakeCoin> for StakeCoin {
    fn from(value: EpochStakeCoin) -> Self {
        Self {
            coin_type: parse_type_tag(&value.coin_type).unwrap_or(TypeTag::Bool),
            balance: value.coin_balance as u64,
            bfc_value: value.bfc_value as u64,
        }
    }
}

pub const RATE_MUL: f64 = 10_000f64;

pub fn get_apy_from_recents(stakes: &[EpochStake]) -> f64 {
    let exchange_rates = stakes
        .into_iter()
        .filter_map(|x| {
            if (x.avg_exchange_rate as f64 / RATE_MUL) < 1.2 {
                Some(x.to_owned())
            } else {
                None
            }
        })
        .take(31)
        .collect::<Vec<_>>();

    if exchange_rates.len() >= 2 {
        // rates are sorted by epoch in descending order.
        let er_e = &exchange_rates[1..];
        // rate e+1
        let er_e_1 = &exchange_rates[..&exchange_rates.len() - 1];
        let dp_count = er_e.len();
        er_e.iter().zip(er_e_1).map(calculate_apy).sum::<f64>() / dp_count as f64
    } else {
        0.0
    }
}

// APY_e = (ER_e+1 / ER_e) ^ 365
fn calculate_apy((stake, stake_1): (&EpochStake, &EpochStake)) -> f64 {
    (stake.avg_exchange_rate as f64 / stake_1.avg_exchange_rate as f64).powf(365.0) - 1.0
}

#[cfg(test)]
mod test {
    use super::{get_apy_from_recents, EpochStake};

    #[test]
    fn test_calculate_apy() {
        let reward_rates = vec![
            18386, 20000, 20000, 20000, 20000, 20000, 20000, 20000, 20000, 18389, 18393, 18398,
            18402, 18407, 18411, 18420, 18424, 18429, 8433, 8438, 8443, 8448, 8453, 8458, 8463,
            8468, 8473, 8478, 8483, 8488, 8493, 8499, 8504, 8510, 8516, 8521, 8527, 8532, 8538,
            8544, 8550, 8556, 8563, 8569, 8575, 8582, 8588, 8594, 8601, 8607, 8614, 8621, 8628,
            8635, 8643, 8650, 8657, 8664, 8671, 8678, 8686, 8694, 8702, 8711, 8719, 8727, 8735,
            8743, 8751, 8759, 9366, 9491,
        ];
        let stakes: Vec<EpochStake> = reward_rates
            .into_iter()
            .map(|x| EpochStake {
                avg_exchange_rate: x,
                ..Default::default()
            })
            .collect();
        let apy = get_apy_from_recents(&stakes);
        println!("apy: {}", apy);
    }
}
