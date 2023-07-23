#[allow(dead_code)]
pub mod default {
    pub const RAINFALL: u8 = 14;
    pub const DHT22_FS_TEMP: &str = "/sys/bus/iio/devices/iio:device0/in_temp_input";
    pub const DHT22_FS_HUMIDITY: &str = "/sys/bus/iio/devices/iio:device0/in_humidityrelative_input";
    pub const BUTTON: u8 = 1;
    pub const BMP280_PATH: &str = "/dev/i2c-1";
    pub const BMP280_ADDRESS: u16 = 0x76;

    pub const WIND_SPEED_PIN: u8 = 21;
    pub const WIND_N: u8 = 20;
    pub const WIND_NE: u8 = 26;
    pub const WIND_E: u8 = 16;
    pub const WIND_SE: u8 = 19;
    pub const WIND_S: u8 = 13;
    pub const WIND_SW: u8 = 12;
    pub const WIND_W: u8 = 6;
    pub const WIND_NW: u8 = 5;


}
