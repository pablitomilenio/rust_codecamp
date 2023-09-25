// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

extern crate regex;

use regex::Regex;

pub fn luhn(cc_number: &str) -> bool {
    let mut result: bool = true;

    let re = Regex::new(r"^[a-zA-Z]+$").unwrap();

    if cc_number.chars().any(|c| c.is_alphabetic()) {
        result = false;
        println!("there are letters");
    }

    let trimmed: String = trim_all_else(cc_number);
    println!("trimmed {}", trimmed);
    println!("len {}", trimmed.len());

    match cc_number.parse::<u32>() {
        Ok(parsed_int) => {
            if !trimmed.len() > 0 {
                result = false;
                println!(
                    "cc number was {} and it failed because of length",
                    cc_number
                );
            }
        }

        Err(_) => {}
    }

    let mut trimmed_array: Vec<u8> = to_int_array(&trimmed);
    trimmed_array.reverse();

    //println!("ta {}", trimmed_array[1]);

    let mut even: Vec<u8> = get_even(&trimmed_array);

    if even.len() == 0 {
        result = false;
    }

    let odd: Vec<u8> = get_odd(&trimmed_array);
    let madd: Vec<u8> = multadd(&odd);

    println!("even {:?}", even);
    println!("odd {:?}", odd);
    println!("madd {:?}", madd);

    even.extend_from_slice(&madd);

    let comb_arr: &Vec<u8> = &even;

    println!("combarr {:?}", comb_arr);

    let sum_num: u8 = comb_arr.iter().sum();

    println!("final array sum {}", &sum_num);

    if sum_num % 10 != 0 {
        result = false;
    }

    println!("the test result is: {}", result);

    result
}

fn trim_all_else(untrimmed: &str) -> String {
    // Input string with leading and trailing whitespace
    let input = untrimmed;

    // Create a regex pattern for matching whitespace
    let whitespace_pattern = Regex::new(r"\D+").unwrap();
    let letters: Regex = Regex::new(r"^[a-z]+$").unwrap();

    // Use replace_all to trim the whitespace
    let trimmed = whitespace_pattern.replace_all(input, "");

    let as_string: String = trimmed.into_owned();

    as_string
}

fn to_int_array(digit_string: &str) -> Vec<u8> {
    let mut digit_array: Vec<u8> = Vec::new();

    for chr in digit_string.chars() {
        if let Some(digit) = chr.to_digit(10) {
            digit_array.push(digit as u8);
        }
    }

    digit_array
}

fn get_even(input: &Vec<u8>) -> Vec<u8> {
    let vector: &Vec<u8> = input;
    let mut even: Vec<u8> = Vec::new();
    let mut odd: Vec<u8> = Vec::new();

    for i in 0..vector.len() {
        if i % 2 == 0 {
            even.push(vector[i]);
        } else {
            odd.push(vector[i]);
        }
    }

    even
}

fn get_odd(input: &Vec<u8>) -> Vec<u8> {
    let vector: &Vec<u8> = input;
    let mut even: Vec<u8> = Vec::new();
    let mut odd: Vec<u8> = Vec::new();

    for i in 0..vector.len() {
        if i % 2 == 0 {
            even.push(vector[i]);
        } else {
            odd.push(vector[i]);
        }
    }

    odd
}

fn multadd(input: &Vec<u8>) -> Vec<u8> {
    let vector: &Vec<u8> = input;

    let mut contents: Vec<u8> = Vec::new();

    for num in vector {
        if num * 2 > 9 {
            let tmp: u8 = num * 2 - 9;
            contents.push(tmp);
        } else {
            contents.push(num * 2);
        }
    }

    contents
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
    assert!(!luhn("foo 0 0"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {
    luhn("foo 0 0");
}
