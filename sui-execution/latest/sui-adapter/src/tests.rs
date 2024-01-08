use crate::{calculate_bfc_to_stable_cost, calculate_stable_to_bfc_cost};

#[test]
fn test_calculate_stable_rate() {
    let cost = 132240;
    let rate = 1000000032u64;
    let result = calculate_bfc_to_stable_cost(cost, rate);
    println!("result: {}", result);
    let result = calculate_stable_to_bfc_cost(result, rate);
    println!("result: {}", result);
    assert_eq!(cost, result);
    let cost_u64_max = u64::MAX;
    let result = calculate_bfc_to_stable_cost(cost_u64_max, rate);
    println!("result: {}", result);
    let result = calculate_stable_to_bfc_cost(result, rate);
    assert_eq!(cost_u64_max, result);
}