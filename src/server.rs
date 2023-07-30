use core::str;
use embedded_svc::{io::Write,http::server::{Request, Method},http::Headers};
use esp_idf_hal::{
    i2c::{I2cConfig, I2cDriver},
    prelude::*,
};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    http::server::{Configuration, EspHttpServer},
};
use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};
use embedded_svc::io::Read;
use esp_idf_hal::delay::FreeRtos;
// If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_sys as _;
use log::info;
use crate::settings::Settings;

use serde::Deserialize;
use serde_json;
use crate::printer::StringToPrint;

static INDEX_HTML: &str = include_str!("index.html");

#[derive(Deserialize,Debug)]
struct StringData {
    string: String,
}

#[derive(Deserialize,Debug)]
struct SettingsData {
    justify: bool,
    characters_per_line: u32,
    min_ms_value: u32,
    max_ms_value: u32,
    backspace_threshold: u32,
}


pub fn server(data_settings: Arc<Mutex<Settings>>, string_to_print: Arc<Mutex<StringToPrint>>) -> Option<()> {

    // Set the HTTP server
    let mut server = EspHttpServer::new(&Configuration::default()).ok()?;

    // Handle the main page
    server.fn_handler("/", Method::Get, |request| {
        let html = index_html();
        let mut response = request.into_ok_response()?;
        response.write_all(INDEX_HTML.as_bytes())?;
        Ok(())
    }).ok()?;

    //Handle posting of the strings
    server.fn_handler("/post/string", Method::Post, move |mut req| {
        let len = req.content_len().unwrap_or(0) as usize;

        // if len > MAX_LEN {
        //     req.into_status_response(413)?
        //         .write_all("Request too big".as_bytes())?;
        //     return Ok(());
        // }

        let mut buf = vec![0; len];
        req.read_exact(&mut buf)?;
        let mut resp = req.into_ok_response()?;

        if let Ok(form) = serde_json::from_slice::<StringData>(&buf) {
            let string_to_print_clone = Arc::clone(&string_to_print);
            update_string(&form, &string_to_print_clone);
            resp.write_all("Succesfull write.".as_bytes())?;
        } else {
            resp.write_all("JSON error".as_bytes())?;
        }
        Ok(())
    }).ok()?;

    let data_settings_clone = data_settings.clone();
    //Handle posting of the strings
    server.fn_handler("/post/settings", Method::Post, move |mut req| {
        let len = req.content_len().unwrap_or(0) as usize;

        // if len > MAX_LEN {
        //     req.into_status_response(413)?
        //         .write_all("Request too big".as_bytes())?;
        //     return Ok(());
        // }

        let mut buf = vec![0; len];
        req.read_exact(&mut buf)?;
        let mut resp = req.into_ok_response()?;

        if let Ok(form) = serde_json::from_slice::<SettingsData>(&buf) {
            update_settings(&form, &data_settings_clone);
            resp.write_all("Succesfull Setting change.".as_bytes())?;
        } else {
            resp.write_all("JSON error".as_bytes())?;
        }
        Ok(())
    }).ok()?;

    loop{
        FreeRtos::delay_ms(1000)
    }
    Some(())
}
fn update_string (string: &StringData,string_data: &Arc<Mutex<StringToPrint>>){
    let mut str_data = string_data.lock().unwrap();
    str_data.string = string.string.clone();
    dbg!("Current new string:",&str_data);
}

fn update_settings (settings: &SettingsData, data_settings: &Arc<Mutex<Settings>>) {
    let mut data = data_settings.lock().unwrap();
    data.justify = settings.justify;
    data.characters_per_line = settings.characters_per_line;
    data.min_ms = settings.min_ms_value;
    data.max_ms = settings.max_ms_value;
    data.chance_threshold_percent = settings.backspace_threshold as f32;
    dbg!("Current new settings:",&data);
}

// Unused
fn templated(content: &str) -> String {
    format!(
        r#"
        <!DOCTYPE hmtl>
        <html>
            <head>
                <met charset="utf-8">
                <title> esp-rs web server</title>
            </head>
            <body>
                {}
            </body>
        </html>
        "#,
            content
    )
}

// Unused
fn index_html() -> String{
    templated("Hello from ESP32!")
}