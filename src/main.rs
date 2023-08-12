use bot::Bot;
use clap::Parser;
use cli::Cli;
use log::info;
use std::time;

mod bot;
mod cli;
mod indicators;
mod klines;
mod util;

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    let api_key = "OawZFDly06Wg6RpLzDQAk7nuiOjEmvOxQdQv6HGcuzngnPqBD3nrGadvcvF58NiA";
    let api_secret = "5E03pZ4sQ22S4CefC7S2Ur5vqc8vRkBNUjEl259dlYhkZklsGBVkDy9z0YkLgeiF";

    let bot = Bot::new(&api_key, &api_secret);

    info!("Starting Bot");
    bot.run(
        &cli.symbol,
        cli.quantity,
        cli.fast_ema,
        cli.slow_ema,
        &cli.interval.to_owned(),
        &time::Duration::from_secs(cli.sleep_duration * 60),
        &cli.initial_side,
    );
}
