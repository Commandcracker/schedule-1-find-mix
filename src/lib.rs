use crate::effects::Effect;
use crate::ingredients::Ingredient;
use rayon::prelude::*;

pub mod effects;
pub mod ingredients;

pub mod product;

#[inline(always)]
pub fn calc_sell_price(base_price: f32, effects: &[Effect]) -> f32 {
    let mut sum: f32 = 0.;
    for effect in effects {
        sum += effect.multiplier()
    }
    base_price + (sum * base_price)
}

#[inline(always)]
pub fn calc_cost(ingredients: &[Ingredient]) -> f32 {
    let mut sum: f32 = 0.;
    for ingredient in ingredients {
        sum += ingredient.price() as f32;
    }
    sum
}

#[inline(always)]
pub fn calc_addictiveness(depth: u32, effects: &[Effect]) -> f32 {
    let mut sum: f32 = 0.;
    for effect in effects {
        sum += effect.addictiveness();
    }
    //TODO: Not tested with Meth and Coke
    if depth > 0 {
        sum += 0.05;
    }
    sum.clamp(0., 1.)
}

#[inline(always)]
pub fn calc_profit(sell_price: f32, cost: f32) -> f32 {
    sell_price - cost
}

pub struct CombinationResult {
    pub profit: f32,
    pub cost: f32,
    pub ingredients: Vec<Ingredient>,
    pub effects: Vec<Effect>,
}

pub fn try_all_combinations_threaded(
    base_price: f32,
    depth: u32,
    initial_effect: Option<Effect>,
    available_ingredients: Vec<Ingredient>,
) -> CombinationResult {
    let current_effects = match initial_effect {
        Some(ref effect) => vec![effect.clone()],
        None => vec![],
    };
    fn generate_combinations(
        ingredients: &[Ingredient],
        depth: u32,
        current_depth: u32,
        current_ingredients: &[Ingredient],
        current_effects: &[Effect],
        base_price: f32,
    ) -> CombinationResult {
        if current_depth == depth {
            let sell_price = calc_sell_price(base_price, current_effects);
            let cost = calc_cost(current_ingredients);
            let profit = calc_profit(sell_price, cost);

            return CombinationResult {
                profit,
                cost,
                ingredients: current_ingredients.to_vec(),
                effects: current_effects.to_vec(),
            };
        }

        ingredients
            .par_iter()
            .map(|ingredient| {
                let mut new_ingredients = Vec::with_capacity(current_ingredients.len() + 1);
                new_ingredients.extend_from_slice(current_ingredients);
                new_ingredients.push(ingredient.clone());

                let new_effects = ingredient.apply(current_effects);

                // Stop if no effect changed
                if new_effects.len() == current_effects.len()
                    && new_effects.iter().zip(current_effects).all(|(a, b)| a == b)
                {
                    let sell_price = calc_sell_price(base_price, &new_effects);
                    let cost = calc_cost(&new_ingredients);
                    let profit = calc_profit(sell_price, cost);

                    return CombinationResult {
                        profit,
                        cost,
                        ingredients: new_ingredients,
                        effects: new_effects,
                    };
                }

                generate_combinations(
                    ingredients,
                    depth,
                    current_depth + 1,
                    &new_ingredients,
                    &new_effects,
                    base_price,
                )
            })
            .max_by(|a, b| {
                a.profit
                    .partial_cmp(&b.profit)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| {
                        a.cost
                            .partial_cmp(&b.cost)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
            })
            .unwrap()
    }

    generate_combinations(
        &available_ingredients,
        depth,
        0,
        &[],
        &current_effects,
        base_price,
    )
}

// Same as try_all_combinations_threaded with `par_iter()` changed to `iter()`
// TODO: find some solution so i dont need to copy this
pub fn try_all_combinations_not_threaded(
    base_price: f32,
    depth: u32,
    initial_effect: Option<Effect>,
    available_ingredients: Vec<Ingredient>,
) -> CombinationResult {
    let current_effects = match initial_effect {
        Some(ref effect) => vec![effect.clone()],
        None => vec![],
    };

    fn generate_combinations(
        ingredients: &[Ingredient],
        depth: u32,
        current_depth: u32,
        current_ingredients: &[Ingredient],
        current_effects: &[Effect],
        base_price: f32,
    ) -> CombinationResult {
        if current_depth == depth {
            let sell_price = calc_sell_price(base_price, current_effects);
            let cost = calc_cost(current_ingredients);
            let profit = calc_profit(sell_price, cost);

            return CombinationResult {
                profit,
                cost,
                ingredients: current_ingredients.to_vec(),
                effects: current_effects.to_vec(),
            };
        }

        ingredients
            .iter()
            .map(|ingredient| {
                let mut new_ingredients = Vec::with_capacity(current_ingredients.len() + 1);
                new_ingredients.extend_from_slice(current_ingredients);
                new_ingredients.push(ingredient.clone());

                let new_effects = ingredient.apply(current_effects);

                // Stop if no effect changed
                if new_effects.len() == current_effects.len()
                    && new_effects.iter().zip(current_effects).all(|(a, b)| a == b)
                {
                    let sell_price = calc_sell_price(base_price, &new_effects);
                    let cost = calc_cost(&new_ingredients);
                    let profit = calc_profit(sell_price, cost);

                    return CombinationResult {
                        profit,
                        cost,
                        ingredients: new_ingredients,
                        effects: new_effects,
                    };
                }

                generate_combinations(
                    ingredients,
                    depth,
                    current_depth + 1,
                    &new_ingredients,
                    &new_effects,
                    base_price,
                )
            })
            .max_by(|a, b| {
                a.profit
                    .partial_cmp(&b.profit)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| {
                        a.cost
                            .partial_cmp(&b.cost)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
            })
            .unwrap()
    }

    generate_combinations(
        &available_ingredients,
        depth,
        0,
        &[],
        &current_effects,
        base_price,
    )
}
