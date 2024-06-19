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
    pub stable_rate: Option<i64>,
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
            let rate = x.avg_exchange_rate as f64 / RATE_MUL;
            if (1.0 / rate) < 1.2 {
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
            25252, 25278, 25303, 25328, 25354, 25379, 25405, 25431, 25456, 25482, 25508, 25534,
            25560, 25586, 25612, 25638, 25664, 25691, 25717, 25743, 25770, 25796, 25823, 25849,
            25876, 25903, 25929, 25956, 25983, 26010, 26037, 26064, 26091, 26118, 26146, 26173,
            26200, 26228, 26255, 26283, 26310, 26338, 26365, 26393, 26421, 26449, 26477, 26505,
            26533, 26561, 26589, 26617, 26646, 26674, 26702, 26731, 26759, 26788, 26817, 26845,
            26874, 26903, 26932, 26961, 26990, 27019, 27048,
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
