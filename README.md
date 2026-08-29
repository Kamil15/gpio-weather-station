# gpio-weather-station

A Raspberry Pi weather station written in Rust. It reads temperature, humidity, air pressure, wind speed, wind direction and rainfall from a set of sensors wired to the Pi, and announces the current conditions out loud using text-to-speech (Festival, in Polish). A button lets you request a spoken forecast on demand, or it can do it automatically on a timer.

## What's inside

The repository contains two independent Rust crates, each with its own `Cargo.toml` (this is not a Cargo workspace):

| Crate | Purpose |
|---|---|
| [`sensor-manager`](sensor-manager) | The main application: polls all sensors in a loop and speaks the forecast |
| [`sensor-test`](sensor-test) | A diagnostic CLI that tests each sensor individually, which is handy for verifying wiring |

## How it works

### Main application (`sensor-manager`)

Everything is orchestrated from a single main loop in [`main.rs`](sensor-manager/src/main.rs) that ticks every 100 ms:

```
                       main loop                                   anemometer thread
   ┌────────────────────────────────────────────────┐      ┌────────────────────────────────┐
   │  every 5 s    → DHT22   (temp, humidity)       │      │ GPIO 21 falling-edge interrupt │
   │  every 10 s   → BMP280  (temp, pressure)       │      │ 2 ticks = 1 rotation           │
   │  every tick   → weathervane (8 direction pins) │      │ speed = rotations/second       │
   │                                                │      │ 10 s of silence → speed = 0    │
   │  button press or auto-timer:                   │      │ publishes via                  │
   │    → snapshot anemometer speed + rainfall      │◄─────┤ Arc<RwLock<AnemometerData>>    │
   └──────────────────────┬─────────────────────────┘      └────────────────────────────────┘
                          │ try_send(ForecastData)
                          │ (zero-buffer channel, skipped if the
                          │  audio thread is still speaking)
                          ▼
   ┌──────────────────────┴─────────────────────────┐
   │                  audio thread                  │
   │                                                │
   │  sh -c "echo ... | festival -"                 │
   │  (Festival TTS, Polish SayText script)         │
   │                                                │
   │  toggles GPIO 11 (low = speaking)              │
   └────────────────────────────────────────────────┘
```

Key components (in [`sensor-manager/src/manager/`](sensor-manager/src/manager)):

- **`manager.rs`**: owns all hardware handles (GPIO pins via `rppal`, the BMP280 over I2C, DHT22 through the kernel IIO driver) and accumulates the latest readings in `current_result`.
- **`anemometer.rs`**: runs its own thread that counts falling-edge interrupts on the speed pin and publishes the speed through an `Arc<RwLock<...>>`. Two low ticks equal one rotation; the calibration factor (1 rotation/s = 16 km/h) lives in `mod.rs`.
- **`audiomanager.rs`**: an audio worker thread fed over an `mpsc::sync_channel(0)`. The main thread uses `try_send`, so a forecast is simply skipped when the previous one is still being spoken. Speaking is done by shelling out to `festival` with a generated Scheme script, and a GPIO output pin signals when transmission is in progress.
- **`mod.rs`**: shared types such as `WindDirection` and `WindSpeed` (with km/h and m/s conversions), plus the weathervane/anemometer pin map.

### Test tool (`sensor-test`)

Flags passed on the command line are turned into a list of `TestTask` implementations ([`test_tasks.rs`](sensor-test/src/test_tasks.rs)) which are then executed one by one, printing readings or errors. Sensor paths and pin numbers are centralized in [`defaults.rs`](sensor-test/src/defaults.rs).

## Hardware

Pin numbers below are BCM numbering, as used by `rppal`.

| Peripheral | Interface | Pin(s) | Notes |
|---|---|---|---|
| DHT22 (temperature & humidity) | kernel IIO driver | n/a | read from `/sys/bus/iio/devices/iio:device0` |
| BMP280 (temperature & pressure) | I2C | bus 1, address `0x76` | `/dev/i2c-1` |
| Anemometer (wind speed) | GPIO in | 21 | pull-up, falling-edge interrupt |
| Weathervane (wind direction) | GPIO in | 20 (N), 26 (NE), 16 (E), 19 (SE), 13 (S), 12 (SW), 6 (W), 5 (NW) | pull-up, active low, one active at a time |
| Rain sensor | GPIO in | 14 | pull-down, active low |
| Speech button | GPIO in | 1 | pull-up, active low |
| Transmission signal | GPIO out | 11 | low while TTS is speaking |

The Pi needs `festival` installed with a Polish voice for the spoken forecast.

## Usage

On the Raspberry Pi:

```console
# speak the forecast when the button is pressed
sensor_manager

# speak automatically, every 300 s (default interval)
sensor_manager --auto-audio

# speak automatically, every 60 s
sensor_manager --auto-audio --interval-audio 60
```

The test tool runs any subset of checks you pass:

```console
sensor_test --rainfall --bmp280 --button --weathervane
sensor_test --dht22 kernel-driver   # read the kernel IIO interface
sensor_test --dht22 direct          # raw one-wire read on GPIO 2 via dht22_pi
```

## Building

The code targets Linux on ARM (Raspberry Pi) and is cross-compiled with [`cross`](https://github.com/cross-rs/cross):

```console
cargo install cross
cd sensor-manager
cross build --release --target armv7-unknown-linux-gnueabihf
```

`Cross.toml` maps each ARM target to a build image; `sensor-manager/docker/` additionally contains custom cross-rs Dockerfiles (used to link the speech-related native libraries on the ARM toolchain).
