use std::cmp::min;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::uart::UartDriver;
use crate::dictionary::ASCII_TO_ERIKA;
use rand::Rng;

#[derive(Debug)]
pub struct StringToPrint {
    pub string: String,
}

pub fn print_to_erika(char: &[u8;1], uart: &UartDriver) {
    uart.write(char).expect("Writing the char over UART has failed");
}

pub fn print_char(char: &char, uart: &UartDriver) {
    print_to_erika(ASCII_TO_ERIKA.get(char).unwrap(), uart);
}

pub fn rhythm_break(min_ms:u32,max_ms:u32){
    let mut rng = rand::thread_rng();
    if min_ms < max_ms {
        FreeRtos::delay_ms(rng.gen_range(min_ms..max_ms));
    }
}

pub fn print_char_mistake(char: &char, uart: &UartDriver, chance_threshold_percent: f32) {
    let mut rng = rand::thread_rng();
    let random: f32 = rng.gen();
    if (random * 100.00) < chance_threshold_percent {
        print_char(&'\u{0008}', uart);
        print_char(char, uart);
    }
}

pub fn print_string(string: &String, uart: &UartDriver) {
    for x in string.chars() {
        print_char(&x,uart);
    }
}

pub fn print_string_rhythm(string: &String, uart: &UartDriver, min_ms:u32,max_ms:u32 ) {
    for x in string.chars() {
        print_char(&x,uart);
        rhythm_break(min_ms,max_ms);
    }
}

pub fn print_string_with_mistakes(string: &String, uart: &UartDriver, chance_threshold_percent: f32) {
    for x in string.chars() {
        print_char_mistake(&x, uart, chance_threshold_percent);
    }
}

pub fn print_string_with_mistakes_and_rhythm(string: &String, uart: &UartDriver, chance_threshold_percent: f32, min_ms:u32, max_ms:u32) {
    for x in string.chars() {
        print_char(&x,uart);
        rhythm_break(min_ms,max_ms);
        print_char_mistake(&x,uart,chance_threshold_percent);
    }
}