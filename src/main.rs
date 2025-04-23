#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use crate::effects::Effect;
use crate::ingredients::Ingredient;
use strum::IntoEnumIterator;
use std::time::Instant;

mod effects;
mod ingredients;

fn calc_sell_price(base_price: f32, effects: &[Effect]) -> f32 {
    let mut sum: f32 = 0.0;
    for effect in effects {
        sum += effect.multiplier()
    }
    base_price + (sum * base_price)
}

fn calc_cost(ingredients: &[Ingredient]) -> f32 {
    let mut sum: f32 = 0.0;
    for ingredient in ingredients {
        sum += ingredient.price() as f32;
    }
    sum
}

#[inline(always)]
fn calc_profit(sell_price: f32, cost: f32) -> f32 {
    sell_price - cost
}


use rayon::prelude::*;

fn try_all_combinations_v3(base_price: f32, depth: u32, initial_effect: Effect) {
    let ingredients: Vec<Ingredient> = Ingredient::iter().collect();
    let current_effects = vec![initial_effect.clone()];

    fn generate_combinations(
        ingredients: &[Ingredient],
        depth: u32,
        current_depth: u32,
        current_ingredients: &[Ingredient],
        current_effects: &[Effect],
        base_price: f32,
        initial_effect: &Effect,
    ) -> (f32, f32, Vec<Ingredient>, Vec<Effect>) {
        if current_depth == depth {
            let sell_price = calc_sell_price(base_price, current_effects);
            let cost = calc_cost(current_ingredients);
            let profit = calc_profit(sell_price, cost);

            return (
                profit,
                cost,
                current_ingredients.to_vec(),
                current_effects.to_vec(),
            );
        }

        ingredients
            .par_iter()
            .map(|ingredient| {
                let mut new_ingredients = Vec::with_capacity(current_ingredients.len() + 1);
                new_ingredients.extend_from_slice(current_ingredients);
                new_ingredients.push(ingredient.clone());

                let new_effects = ingredient.apply(current_effects);

                generate_combinations(
                    ingredients,
                    depth,
                    current_depth + 1,
                    &new_ingredients,
                    &new_effects,
                    base_price,
                    initial_effect,
                )
            })
            .max_by(|a, b| {
                a.0.partial_cmp(&b.0)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            })
            .unwrap()
    }

    let result = generate_combinations(
        &ingredients,
        depth,
        0,
        &[],
        &current_effects,
        base_price,
        &initial_effect,
    );

    let final_sell_price = calc_sell_price(base_price, &result.3);

    // TODO: Move out
    println!("Profit: ${}", result.0);
    println!("Cost: ${}", result.1);
    println!("Sell Price: ${}", final_sell_price);
    println!(
        "Ingredients: {}",
        result
            .2
            .iter()
            .map(|i| i.name())
            .collect::<Vec<_>>()
            .join(" → ")
    );
    println!(
        "Effects: {}",
        result
            .3
            .iter()
            .map(|e| e.colord_name())
            .collect::<Vec<_>>()
            .join(", ")
    );
}

use rayon::prelude::*;

fn try_all_combinations_v2(base_price: f32, depth: usize, initial_effect: Effect) {
    let ingredients: Vec<Ingredient> = Ingredient::iter().collect();
    let mut best_combo = (f32::MIN, f32::MAX, Vec::new(), Vec::new());

    fn generate_combinations(
        ingredients: &[Ingredient],
        depth: usize,
        current_depth: usize,
        current_ingredients: Vec<Ingredient>,
        current_effects: Vec<Effect>,
        base_price: f32,
        initial_effect: &Effect,
    ) -> (f32, f32, Vec<Ingredient>, Vec<Effect>) {
        if current_depth == depth {
            let sell_price = calc_sell_price(base_price, &current_effects);
            let cost = calc_cost(&current_ingredients);
            let profit = calc_profit(sell_price, cost);

            return (profit, cost, current_ingredients, current_effects);
        }

        ingredients
            .par_iter()
            .map(|ingredient| {
                let mut new_ingredients = current_ingredients.clone();
                new_ingredients.push(ingredient.clone());

                let mut new_effects = current_effects.clone();
                new_effects = ingredient.apply(&new_effects);

                generate_combinations(
                    ingredients,
                    depth,
                    current_depth + 1,
                    new_ingredients,
                    new_effects,
                    base_price,
                    initial_effect,
                )
            })
            .max_by(|a, b| {
                a.0.partial_cmp(&b.0)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            })
            .unwrap_or((f32::MIN, f32::MAX, Vec::new(), Vec::new()))
    }

    let current_ingredients = Vec::new();
    let current_effects = vec![initial_effect.clone()];

    best_combo = generate_combinations(
        &ingredients,
        depth,
        0,
        current_ingredients,
        current_effects,
        base_price,
        &initial_effect,
    );

    let final_sell_price = calc_sell_price(base_price, &best_combo.3);

    println!("Profit: ${}", best_combo.0);
    println!("Cost: ${}", best_combo.1);
    println!("Sell Price: ${}", final_sell_price);
    println!(
        "Ingredients: {}",
        best_combo
            .2
            .iter()
            .map(|i: &Ingredient| i.name())
            .collect::<Vec<_>>()
            .join(" → ")
    );
    println!(
        "Effects: {}",
        best_combo
            .3
            .iter()
            .map(|e| e.colord_name())
            .collect::<Vec<_>>()
            .join(", ")
    );
}

fn try_all_combinations(base_price: f32, depth: usize, initial_effect: Effect) {
    let ingredients: Vec<Ingredient> = Ingredient::iter().collect();
    let mut best_combo = (f32::MIN, f32::MAX, Vec::new(), Vec::new());

    fn generate_combinations(
        ingredients: &[Ingredient],
        depth: usize,
        current_depth: usize,
        current_ingredients: &mut Vec<Ingredient>,
        current_effects: &mut Vec<Effect>,
        base_price: f32,
        initial_effect: &Effect,
        best_combo: &mut (f32, f32, Vec<Ingredient>, Vec<Effect>),
    ) {
        if current_depth == depth {
            let sell_price = calc_sell_price(base_price, &*current_effects);
            let cost = calc_cost(&*current_ingredients);
            let profit = calc_profit(sell_price, cost);

            if profit > best_combo.0 || (profit == best_combo.0 && cost < best_combo.1) {
                *best_combo = (
                    profit,
                    cost,
                    current_ingredients.clone(),
                    current_effects.clone(),
                );
            }
            return;
        }

        for ingredient in ingredients {
            current_ingredients.push(ingredient.clone());
            let original_effects = current_effects.clone();
            *current_effects = ingredient.apply(&*current_effects);

            generate_combinations(
                ingredients,
                depth,
                current_depth + 1,
                current_ingredients,
                current_effects,
                base_price,
                initial_effect,
                best_combo,
            );

            current_effects.clear();
            current_effects.extend(original_effects);
            current_ingredients.pop();
        }
    }

    let mut current_ingredients = Vec::new();
    let mut current_effects = vec![initial_effect.clone()];
    generate_combinations(
        &ingredients,
        depth,
        0,
        &mut current_ingredients,
        &mut current_effects,
        base_price,
        &initial_effect,
        &mut best_combo,
    );

    let final_sell_price = calc_sell_price(base_price, &*best_combo.3);

    println!("Profit: ${}", best_combo.0);
    println!("Cost: ${}", best_combo.1);
    println!("Sell Price: ${}", final_sell_price);
    println!(
        "Ingredients: {}",
        best_combo.2
            .iter()
            .map(|i: &Ingredient| i.name())
            .collect::<Vec<_>>()
            .join(" → ")
    );
    println!(
        "Effects: {}",
        best_combo
            .3
            .iter()
            .map(|e| e.colord_name())
            .collect::<Vec<_>>()
            .join(", ")
    );
}

use clap::{Arg, Command};

enum Product {
    OgKush,
    SourDiesel,
    GreenCrack,
    GrandaddyPurple,
    Meth,
    Coke
}

impl Product {
    const fn value(&self) -> f32 {
        match self {
            Product::Meth => 70.0,
            Product::Coke => 150.0,
            _ => 35.0,
        }
    }
    const fn effect(&self) -> Option<Effect> {
        match self {
            Product::OgKush => Some(Effect::Calming),
            Product::SourDiesel => Some(Effect::Refreshing),
            Product::GreenCrack => Some(Effect::Energizing),
            Product::GrandaddyPurple => Some(Effect::Sedating),
            _ => None
        }
    }
    const fn name(&self) -> &'static str {
        match self {
            Product::OgKush => "OG Kush",
            Product::SourDiesel => "Sour Diesel",
            Product::GreenCrack => "Green Crack",
            Product::GrandaddyPurple => "Grandaddy Purple",
            Product::Meth => "Meth",
            Product::Coke => "Coke"
        }
    }
    const fn from_u8(id: u8) -> Option<&'static Self> {
        const PRODUCTS: [Product; 6] = [
            Product::OgKush,
            Product::SourDiesel,
            Product::GreenCrack,
            Product::GrandaddyPurple,
            Product::Meth,
            Product::Coke,
        ];

        if id < PRODUCTS.len() as u8 {
            Some(&PRODUCTS[id as usize])
        } else {
            None
        }
    }
}

impl std::fmt::Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

fn main() {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::new("product")
            .help(
                "Product ID (0–5):\n\
                    0 = OG Kush\n\
                    1 = Sour Diesel\n\
                    2 = GreenCrack\n\
                    3 = Grandaddy Purple\n\
                    4 = Meth\n\
                    5 = Coke"
            )
            .required(true)
            .num_args(1)
            .index(1)
            .value_parser(clap::value_parser!(u8).range(0..=5))
        )
        .arg(Arg::new("depth")
            .help("Amount of Ingredients")
            .required(true)
            .num_args(1)
            .index(2)
            .value_parser(clap::value_parser!(u32))
        )
        .get_matches();

    let product = Product::from_u8(*matches.get_one::<u8>("product").unwrap()).unwrap();
    let depth: u32 = *matches.get_one::<u32>("depth").unwrap();

    println!("Product: {}", product);
    print!("Trying 16^{} combinations...\r", depth); // 16u128.pow(depth as u32)
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    let start_time = Instant::now();
    try_all_combinations_v3(product.value(), depth, product.effect().unwrap());
    let elapsed_time = start_time.elapsed().as_secs_f64();
    println!("Execution Time: {} seconds", elapsed_time);
}
