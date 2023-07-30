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
        for (index, line) in paragraph.iter().enumerate() {
            //Only justify if it isn't the last line of the paragraph
            if data.justify && paragraph.len() < index + 1 {
                print_string_with_mistakes_and_rhythm(&justify_line(line, data), uart, data.chance_threshold_percent, data.min_ms, data.max_ms);
            } else {
                print_string_with_mistakes_and_rhythm(&line.to_string(), uart, data.chance_threshold_percent, data.min_ms, data.max_ms);
            }
            print_string(&"\n".to_string(), uart);
        }
    }
}

fn justify_line(line: &str, data: &MutexGuard<'_, Settings>) -> String {
    // let mut rng = rand::thread_rng();
    let mut line = line.to_string();
    // let mut line_length = line.chars().count();
    let mut spaces_to_add = data.characters_per_line as usize - line_length;
    let white_spaces = line.matches(" ").count();

    //if there are no white spaces, return the line as it is
    if white_spaces == 0 {
        return line;
    }

    let space_to_add_per_white_space = spaces_to_add / white_spaces;
    let mut extra_spaces = spaces_to_add % white_spaces;
    let mut index = 0;

    //Add spaces per white space
    //And add extra spaces if of the modulo of the division
    //This results in an even distribution of spaces (beginning of the sentence will have larger spacing
    // than the end of the sentence if the modulo is not 0)
    while index < line.len() {
        if line.chars().nth(index).unwrap() == ' ' {
            while extra_spaces > 0 {
                line.insert(index, ' ');
                extra_spaces -= 1;
            }
            for _ in 0..space_to_add_per_white_space {
                line.insert(index, ' ');
            }
        }
        index += 1;
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