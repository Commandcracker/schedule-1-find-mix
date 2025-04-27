#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
use clap::{Arg, ArgAction, Command};
use find_mix::ingredients::Ingredient;
use find_mix::product::Product;
use find_mix::{
    CombinationResult, calc_addictiveness, calc_sell_price, try_all_combinations_not_threaded,
    try_all_combinations_threaded,
};
use std::time::Instant;
use strum::IntoEnumIterator;

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

fn main() {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg_required_else_help(true)
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
        .arg(
            Arg::new("no-threading")
                .help("Will disable threading")
                .long("no-threading")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("ingredients")
                .help(
                    "List of ingredients to be deactivated (separated by comma)\n\
                    Ingredient ID (0–15):\n\
                    0  = Cuke\n\
                    1  = Banana\n\
                    2  = Paracetamol\n\
                    3  = Donut\n\
                    4  = Viagra\n\
                    5  = Mouth Wash\n\
                    6  = Flu Medicine\n\
                    7  = Gasoline\n\
                    8  = Energy Drink\n\
                    9  = Motor Oil\n\
                    10 = Mega Bean\n\
                    11 = Chili\n\
                    12 = Battery\n\
                    13 = Iodine\n\
                    14 = Addy\n\
                    15 = Horse Semen",
                )
                .long("disable")
                .value_delimiter(',')
                .value_parser(clap::value_parser!(u8).range(0..=15)),
        )
        .get_matches();

    let product = Product::from_u8(*matches.get_one::<u8>("product").unwrap()).unwrap();
    let depth: u32 = *matches.get_one::<u32>("depth").unwrap();

    let disabled_ingredients = matches
        .get_many::<u8>("ingredients")
        .into_iter()
        .flatten()
        .filter_map(|&num| Ingredient::from_u8(num))
        .collect::<Vec<_>>();

    let available_ingredients: Vec<Ingredient> = Ingredient::iter()
        .filter(|ingredient| !disabled_ingredients.contains(&ingredient))
        .collect();

    println!("Product: {}", product);
    print!(
        "Trying {}^{} combinations...\r",
        available_ingredients.len(),
        depth
    );
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    let start_time = Instant::now();

    let result = if matches.get_flag("no-threading") {
        try_all_combinations_not_threaded(
            product.value(),
            depth,
            product.effect(),
            available_ingredients,
        )
    } else {
        try_all_combinations_threaded(
            product.value(),
            depth,
            product.effect(),
            available_ingredients,
        )
    };

    let elapsed_time = start_time.elapsed().as_secs_f64();
    print!("\x1b[2K"); // Clear line
    print_res(product.value(), &result);
    println!(
        "Addictiveness: {}%",
        calc_addictiveness(depth, result.effects.as_slice()) * 100.0
    );
    //println!("Addictiveness: {}%", (calc_addictiveness(depth, result.effects.as_slice())*100.0).floor());
    println!("Execution Time: {} seconds", elapsed_time);
}
