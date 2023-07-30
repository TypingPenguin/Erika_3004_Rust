use std::cmp::min;
use std::sync::{Arc, Mutex, MutexGuard};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::uart::UartDriver;
use log::info;
use crate::dictionary::ASCII_TO_ERIKA;
use rand::Rng;
use crate::settings::Settings;

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

pub fn print_anything(string: &String, uart: &UartDriver, data: &MutexGuard<'_, Settings>){
    let parsed_text = parse_text(string, data);

    for paragraph in parsed_text {
        for line in paragraph {
            if data.justify {
                print_string_with_mistakes_and_rhythm(&justify_line(line, data), uart, data.chance_threshold_percent, data.min_ms, data.max_ms);
            } else {
                print_string_with_mistakes_and_rhythm(&line.to_string(), uart, data.chance_threshold_percent, data.min_ms, data.max_ms);
            }
            print_string(&"\n".to_string(), uart);
        }
    }
}

fn justify_line(line: &str, data: &MutexGuard<'_, Settings>) -> String {
    let mut rng = rand::thread_rng();
    let mut line = line.to_string();
    let mut line_length = line.chars().count();
    let mut spaces_to_add = data.characters_per_line as usize - line_length;

    while spaces_to_add > 0 {



        let mut spaces_to_add_this_iteration = min(rng.gen_range(1..4), spaces_to_add);
        let mut index = 0;

        while spaces_to_add_this_iteration > 0 {
            if line.len()>index {
                if line.chars().nth(index).unwrap() == ' ' {
                    line.insert(index, ' ');
                    spaces_to_add_this_iteration -= 1;
                }
            } else {
                line.push(' ');
                spaces_to_add_this_iteration -= 1;
            }
            index += 1;
        }
        line_length = line.chars().count();
        spaces_to_add = data.characters_per_line as usize - line_length;
    }
    line
}

fn parse_text<'a>(string: &'a String, data: &'a MutexGuard<'a, Settings>) -> Vec<Vec<&'a str>> {
    let paragraphs: Vec<&str> = string.split("\n").collect();
    let mut parsed_text: Vec<Vec<&str>> = vec![vec![]; paragraphs.len()];
    let char_max_length = (data.characters_per_line + 1) as usize;

    //for elements in paragraphs
    for (paragraph_index, mut paragraph) in paragraphs.into_iter().enumerate() {
        let mut start_index = 0;

        while start_index < paragraph.len() {
            let end_index = if start_index + char_max_length >= paragraph.len() {
                paragraph.len()
            } else {
                start_index + char_max_length - paragraph[start_index..start_index + char_max_length].chars().rev().take_while(|&c| c != ' ').count()
            };

            let line = &paragraph[start_index..end_index].trim_end();
            // println!("Line {}: {}", paragraph_index + 1, line);

            start_index = end_index;
            parsed_text[paragraph_index].push(line.clone());
        }
    }
    parsed_text
}