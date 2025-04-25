use find_mix::effects::Effect;
use find_mix::ingredients::Ingredient;
use find_mix::{calc_addictiveness, calc_cost, calc_profit, calc_sell_price};
use strum::IntoEnumIterator;
#[test]
fn test_calc_sell_price() {
    let all_effects: Vec<Effect> = Effect::iter().collect();
    let result = calc_sell_price(35., all_effects.as_slice());
    assert_eq!(result, 353.49997); // 353.5
}

#[test]
fn test_calc_cost_positive() {
    let all_ingredient: Vec<Ingredient> = Ingredient::iter().collect();
    let result = calc_cost(all_ingredient.as_slice());
    assert_eq!(result, 88.)
}
#[test]
fn test_calc_cost_negative() {
    let result = calc_cost(vec![].as_slice());
    assert_eq!(result, 0.)
}

#[test]
fn test_calc_profit_zero() {
    let result = calc_profit(50., 50.);
    assert_eq!(result, 0.);
}
#[test]
fn test_calc_profit_positive() {
    let result = calc_profit(50., 10.);
    assert_eq!(result, 40.);
}
#[test]
fn test_calc_profit_negative() {
    let result = calc_profit(50., 60.);
    assert_eq!(result, -10.);
}

#[test]
fn test_calc_addictiveness() {
    let effects = vec![Effect::Sedating];
    let result = calc_addictiveness(1, effects.as_slice());
    assert_eq!(result, 0.05);
}
