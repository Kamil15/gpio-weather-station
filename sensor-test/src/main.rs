use std::error::Error;

use bmp280::Bmp280Builder;
use clap::Parser;
use cli::Cli;
use defaults::default;
use rppal::gpio::Gpio;
use test_tasks::*;

mod cli;
mod defaults;
mod test_tasks;

fn main() {
    let cli = Cli::parse();
    let work = WorkBuilder::new().from_cli(cli).build();
    work.start();
}

struct WorkBuilder {
    rainfall: Option<u8>,
    button: Option<u8>, //todo
    bmp280: Option<(u16, String)>,
    dht22: cli::Dht22Option,
    weathervane: Option<Vec<(u8, String)>>,
}

impl WorkBuilder {
    pub fn new() -> Self {
        Self {
            rainfall: None,
            bmp280: None,
            dht22: cli::Dht22Option::None,
            button: None,
            weathervane: None,
        }
    }

    pub fn from_cli(&mut self, cli: Cli) -> &mut Self {
        cli.rainfall
            .then(|| self.rainfall = Some(default::RAINFALL));
        cli.bmp280
            .then(|| self.bmp280 = Some((default::BMP280_ADDRESS, default::BMP280_PATH.into())));
        self.dht22 = cli.dht22.clone();
        cli.button.then(|| self.button = Some(default::BUTTON));

        cli.weathervane.then(|| {
            self.weathervane = Some(vec![
                (default::WIND_N, "N".into()),
                (default::WIND_NE, "NE".into()),
                (default::WIND_E, "E".into()),
                (default::WIND_SE, "SE".into()),
                (default::WIND_S, "S".into()),
                (default::WIND_SW, "SW".into()),
                (default::WIND_W, "W".into()),
                (default::WIND_NW, "NW".into()),
            ]);
        });

        self
    }

    pub fn build(&self) -> Work {
        let mut tasks: Vec<Box<dyn TestTask>> = Vec::new();
        match self.rainfall.clone() {
            Some(x) => tasks.push(Box::new(TestRainFall(x))),
            _ => (),
        }

        match self.bmp280.clone() {
            Some(x) => tasks.push(Box::new(TestBMP280(x.0, x.1))),
            _ => (),
        }

        match self.dht22.clone() {
            cli::Dht22Option::Direct => tasks.push(Box::new(TestDHT22Direct)),
            cli::Dht22Option::KernelDriver => tasks.push(Box::new(TestDHT22Kernel)),
            _ => (),
        }

        match self.button.clone() {
            Some(x) => tasks.push(Box::new(TestButton(x))),
            _ => (),
        }

        match self.weathervane.clone() {
            Some(pins) => tasks.push(Box::new(TestWeathervane(pins))),
            _ => (),
        }

        Work { tasks }
    }
}

struct Work {
    tasks: Vec<Box<dyn TestTask>>,
}

impl Work {
    pub fn start(&self) {
        self.tasks.iter().for_each(|x| {
            println!("-----------------------");
            let result = x.commit();
            match result {
                Err(err) => println!("{:?}", err),
                _ => (),
            }
        });
    }
}
