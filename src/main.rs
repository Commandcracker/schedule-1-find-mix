#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
use clap::{Arg, Command};
use find_mix::product::Product;
use find_mix::{CombinationResult, calc_addictiveness, calc_sell_price, try_all_combinations};
use std::time::Instant;

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
    let result = try_all_combinations(product.value(), depth, product.effect());
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
