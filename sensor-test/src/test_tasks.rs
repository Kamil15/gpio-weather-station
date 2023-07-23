use std::error::Error;

use bmp280::Bmp280Builder;
use rppal::gpio::Gpio;

use crate::defaults::default;

pub trait TestTask {
    fn commit(&self) -> Result<(), Box<dyn Error>>;
}

pub struct TestRainFall(pub u8);
pub struct TestBMP280(pub u16, pub String);
pub struct TestDHT22Direct;
pub struct TestDHT22Kernel;
pub struct TestButton(pub u8);
pub struct TestWeathervane(pub Vec<(u8, String)>);

impl TestTask for TestRainFall {
    fn commit(&self) -> Result<(), Box<dyn Error>> {
        println!("Rainfall GPIO Pin: {}", self.0);
        let input = Gpio::new()?.get(self.0)?.into_input_pulldown().is_high();
        println!("Input state: {}", input);
        Ok(())
    }
}
impl TestTask for TestBMP280 {
    fn commit(&self) -> Result<(), Box<dyn Error>> {
        println!("BMP280 GPIO Pin: {}, {}", self.0, self.1);
        let mut contoller = Bmp280Builder::new()
            .address(self.0)
            .path(default::BMP280_PATH)
            .build()?;
        println!("pressure_kpa: {}", contoller.pressure_kpa()?);
        println!("temperature_celsius: {}", contoller.temperature_celsius()?);
        Ok(())
    }
}
impl TestTask for TestDHT22Kernel {
    fn commit(&self) -> Result<(), Box<dyn Error>> {
        println!("TestDHT22Kernel");
        let mut dht22_temp: Option<f32> = None;
        let mut dht22_humidity: Option<f32> = None;
        let _ = std::fs::read_to_string(default::DHT22_FS_TEMP).and_then(|it| {
            let _ = it.trim_end().parse::<f32>().and_then(|parsed| {
                dht22_temp = Some(parsed / 1000.0);
                Ok(())
            });
            Ok(())
        });

        let _ = std::fs::read_to_string(default::DHT22_FS_HUMIDITY).and_then(|it| {
            let _ = it.trim_end().parse::<f32>().and_then(|parsed| {
                dht22_humidity = Some(parsed / 1000.0);
                Ok(())
            });
            Ok(())
        });
        println!("temperature_celsius: {:?}", dht22_temp);
        println!("humidity: {:?}", dht22_humidity);
        Ok(())
    }
}
impl TestTask for TestDHT22Direct {
    fn commit(&self) -> Result<(), Box<dyn Error>> {
        println!("TestDHT22Direct");
        match dht22_pi::read(2) {
            Ok(res) => println!(
                "temperature_celsius: {}, humidity: {}",
                res.temperature, res.humidity
            ),
            Err(error) => println!("{:?}", error),
        };
        Ok(())
    }
}
impl TestTask for TestButton {
    fn commit(&self) -> Result<(), Box<dyn Error>> {
        println!("TestButton GPIO Pin: {}", self.0);
        let input = Gpio::new()?.get(self.0)?.into_input_pullup().is_high();
        println!("Input state: {}", input);
        Ok(())
    }
}


impl TestTask for TestWeathervane {
    fn commit(&self) -> Result<(), Box<dyn Error>> {
        let gpio = Gpio::new()?;
        println!("TestWeathervane");
        let mut inputs = Vec::new();

        for pin in self.0.as_slice() {
            let input = gpio.get(pin.0)?.into_input_pullup();
            inputs.push((input, pin.1.clone()));
        }

        println!("[Inputs]");

        for input in inputs.iter() {
            println!(
                "{}, {}: {}",
                input.0.pin(),
                input.1,
                input.0.is_high() as u8
            );
        }

        Ok(())
    }
}
