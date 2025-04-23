#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use crate::{effects::Effect, ingredients::Ingredient};
use clap::{Arg, Command};
use rayon::prelude::*;
use std::time::Instant;
use strum::IntoEnumIterator;

mod effects;
mod ingredients;

fn calc_sell_price(base_price: f32, effects: &[Effect]) -> f32 {
    let mut sum: f32 = 0.;
    for effect in effects {
        sum += effect.multiplier()
    }
    base_price + (sum * base_price)
}

fn calc_cost(ingredients: &[Ingredient]) -> f32 {
    let mut sum: f32 = 0.;
    for ingredient in ingredients {
        sum += ingredient.price() as f32;
    }
    sum
}

fn calc_addictiveness(depth: u32, effects: &[Effect]) -> f32 {
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
fn calc_profit(sell_price: f32, cost: f32) -> f32 {
    sell_price - cost
}

struct CombinationResult {
    profit: f32,
    cost: f32,
    ingredients: Vec<Ingredient>,
    effects: Vec<Effect>,
}

fn try_all_combinations_v3(
    base_price: f32,
    depth: u32,
    initial_effect: Option<Effect>,
) -> CombinationResult {
    let ingredients: Vec<Ingredient> = Ingredient::iter().collect();
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

    generate_combinations(&ingredients, depth, 0, &[], &current_effects, base_price)
}

fn print_res(base_price: f32, result: &CombinationResult) {
    let final_sell_price = calc_sell_price(base_price, &result.effects);
    println!("Profit: ${}", result.profit);
    println!("Cost: ${}", result.cost);
    println!("Sell Price: ${}", final_sell_price);
    println!(
        "Ingredients: {}",
        result
            .ingredients
            .iter()
            .map(|i| i.name())
            .collect::<Vec<_>>()
            .join(" → ")
    );
    println!(
        "Effects: {}",
        result
            .effects
            .iter()
            .map(|e| e.colord_name())
            .collect::<Vec<_>>()
            .join(", ")
    );
}

enum Product {
    OgKush,
    SourDiesel,
    GreenCrack,
    GrandaddyPurple,
    Meth,
    Coke,
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
            _ => None,
        }
    }
    const fn name(&self) -> &'static str {
        match self {
            Product::OgKush => "OG Kush",
            Product::SourDiesel => "Sour Diesel",
            Product::GreenCrack => "Green Crack",
            Product::GrandaddyPurple => "Grandaddy Purple",
            Product::Meth => "Meth",
            Product::Coke => "Coke",
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
        .arg(
            Arg::new("product")
                .help(
                    "Product ID (0–5):\n\
                    0 = OG Kush\n\
                    1 = Sour Diesel\n\
                    2 = GreenCrack\n\
                    3 = Grandaddy Purple\n\
                    4 = Meth\n\
                    5 = Coke",
                )
                .required(true)
                .num_args(1)
                .index(1)
                .value_parser(clap::value_parser!(u8).range(0..=5)),
        )
        .arg(
            Arg::new("depth")
                .help("Amount of Ingredients")
                .required(true)
                .num_args(1)
                .index(2)
                .value_parser(clap::value_parser!(u32)),
        )
        .get_matches();

    let product = Product::from_u8(*matches.get_one::<u8>("product").unwrap()).unwrap();
    let depth: u32 = *matches.get_one::<u32>("depth").unwrap();

    println!("Product: {}", product);
    print!("Trying 16^{} combinations...\r", depth); // 16u128.pow(depth as u32)
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    let start_time = Instant::now();
    let result = try_all_combinations_v3(product.value(), depth, product.effect());
    let elapsed_time = start_time.elapsed().as_secs_f64();
    print!("\x1b[2K"); // Clear line
    print_res(product.value(), &result);
    println!("Addictiveness: {}%", calc_addictiveness(depth, result.effects.as_slice())*100.0);
    println!("Execution Time: {} seconds", elapsed_time);
}
