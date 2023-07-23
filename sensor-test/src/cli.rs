use clap::{self, Parser};


#[derive(Debug, Parser)]
#[command(long_about = None)]
pub struct Cli {
    #[arg(long, default_value_t = false)]
    pub rainfall: bool,
    #[arg(long, default_value_t = false)]
    pub bmp280: bool,
    #[arg(long, default_value_t = false)]
    pub button: bool,
    #[arg(long, default_value_t = false)]
    pub weathervane: bool,
    #[arg(value_enum, long, default_value_t = Dht22Option::None)]
    pub dht22: Dht22Option,
}


#[derive(clap::ValueEnum, Debug, Clone, Copy)]
pub enum Dht22Option {
    KernelDriver,
    Direct,
    None,
}