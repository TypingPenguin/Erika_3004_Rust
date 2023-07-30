mod dictionary;
mod printer;
mod server;
mod settings;

use anyhow::Result;
use core::str;
use embedded_svc::{http::Method, io::Write};
use esp_idf_hal::{uart::*,
    i2c::{I2cConfig, I2cDriver},
    prelude::*,
    gpio,
};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    http::server::{Configuration, EspHttpServer},
};
use shtcx::{self, shtc3, PowerMode};
use std::{sync::{Arc, Mutex}, thread::sleep, time::Duration, fmt, thread};
use esp_idf_hal::delay::{BLOCK, FreeRtos, NON_BLOCK};
use esp_idf_hal::gpio::{Gpio1, Gpio16, Gpio17, Gpio22, Gpio23};
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::uart::config::{DataBits, FlowControl, Parity, StopBits};
use esp_idf_hal::uart::config::DataBits::DataBits8;
use wifi::wifi;
// If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_sys as _;
use log::info;
use crate::dictionary::ASCII_TO_ERIKA;
use crate::printer::StringToPrint;
use crate::settings::Settings;


const CR: u8 = 13;

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

pub struct MyPeripherals {
    peripherals: Peripherals,
    sysloop: EspSystemEventLoop,
}

fn main() -> Result<()> {
    //Initialize board
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let data_settings = Arc::new(Mutex::new(Settings {
        justify: false,
        characters_per_line: 65,
        min_ms: 0,
        max_ms: 0,
        chance_threshold_percent: 0.0,
    }));

    let string_to_print = Arc::new(Mutex::new(StringToPrint {
        string: "".to_string(),
    }));

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take().unwrap();

    //Set-up wifi
    let app_config = CONFIG;
    let _wifi = wifi(app_config.wifi_ssid, app_config.wifi_psk,peripherals.modem,sysloop);

    let data_settings_server = data_settings.clone();
    let string_to_print_server = string_to_print.clone();

    let server_server_handle = thread::spawn(move || {
        server::server(data_settings_server, string_to_print_server);
    });

    //Make a Uart driver
    let uart = init_uart(peripherals.pins.gpio17, peripherals.pins.gpio16, peripherals.uart2, peripherals.pins.gpio22, peripherals.pins.gpio23);

    // let testString = "Butthole";

    // printer::print_string_with_mistakes_and_rhythm(testString, &uart, 0.0,0,500);


    loop{

        //Check if there is some data
        {
            let mut string_data = string_to_print.lock().unwrap();
            if string_data.string.len() > 0 {
                let mut data = data_settings.lock().unwrap();
                printer::print_anything(&string_data.string, &uart, &data);
            }
            string_data.string = "".to_string();
        }
        FreeRtos::delay_ms(1000)
    }

}

fn init_uart(tx: Gpio17, rx: Gpio16, UART2: UART2, CTS: Gpio22, RTS: Gpio23) -> UartDriver<'static> {

    let config = config::Config::new().baudrate(Hertz(1200));
    let config2 = config::Config {
        baudrate: Hertz(1200),
        data_bits: DataBits::DataBits8,
        parity: Parity::ParityNone,
        stop_bits: StopBits::STOP1,
        flow_control: FlowControl::CTS,
        flow_control_rts_threshold: 0,
        source_clock: Default::default(),
        intr_flags: Default::default(),
    };

    let uart = UartDriver::new(UART2, tx, rx, Option::Some(CTS), Option::Some(RTS), &config2).unwrap();
    uart
}

fn templated(content: impl AsRef<str>) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <title>esp-rs web server</title>
    </head>
    <body>
        {}
    </body>
</html>
"#,
        content.as_ref()
    )
}

fn index_html() -> String {
    templated("Hello from ESP32-C3!")
}

fn temperature(val: f32) -> String {
    templated(format!("Chip temperature: {:.2}°C", val))
}
