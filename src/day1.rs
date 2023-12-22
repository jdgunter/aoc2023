
pub fn part1(lines: &Vec<String>) {
    let mut calibration_sum = 0;
    for line in lines {
        let mut first_digit = 0;
        let mut second_digit = 0;
        for char in line.chars() {
            if char.is_digit(10) {
                first_digit = char.to_digit(10).unwrap();
                break;
            }
        }
        for char in line.chars().rev() {
            if char.is_digit(10) {
                second_digit = char.to_digit(10).unwrap();
                break
            }
        }
        calibration_sum += first_digit * 10 + second_digit;
    }
    println!("{}", calibration_sum);
}


const DIGIT_NAMES: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];


fn check_for_digit_name_match(line_bytes: &[u8], index: usize, reversed: bool) -> Option<u8> {
    // Check if the line contains a digit name starting at the given index.
    // If reversed = true, will instead check for a digit name ending at the given index.
    let mut name_matches;
    let digit_names_iterator: Box<dyn Iterator<Item=(usize, &&str)>> = if reversed { 
        Box::new(DIGIT_NAMES.iter().enumerate().rev()) 
    } else { 
        Box::new(DIGIT_NAMES.iter().enumerate()) 
    };
    for (digit, name) in digit_names_iterator {
        name_matches = true;
        let name_chars_iterator: Box<dyn Iterator<Item=(usize, &u8)>> = if reversed {
            Box::new(name.as_bytes().iter().rev().enumerate())
        } else {
            Box::new(name.as_bytes().iter().enumerate())
        };
        for (offset, char) in name_chars_iterator {
            let char_at_offset = if reversed { line_bytes[index-offset] } else { line_bytes[index+offset] };
            if char_at_offset != *char {
                name_matches = false;
                break;
            }
        }
        if name_matches {
            // println!("NAME: {} DIGIT: {}", name, digit);
            return Some(digit as u8);
        }
    }
    return None;
}


pub fn part2(lines: &Vec<String>) {
    let mut calibration_sum = 0u64;
    for line in lines {
        let mut first_digit = 0;
        let mut second_digit = 0;
        let line_bytes = line.as_bytes();
        for i in 0..line_bytes.len() {
            let char = line_bytes[i];
            if char.is_ascii_digit() {
                first_digit = char - ('0' as u8);
                // println!("{}", first_digit);
                break;
            }
            else {
                match check_for_digit_name_match(line_bytes, i, false) {
                    Some(digit) => {
                        first_digit = digit;
                        break;
                    }
                    _ => (),
                }
            }
        }
        for i in (0..line_bytes.len()).rev() {
            let char = line_bytes[i];
            if char.is_ascii_digit() {
                second_digit = char - ('0' as u8);
                // println!("{}", second_digit);
                break;
            }
            else {
                match check_for_digit_name_match(line_bytes, i, true) {
                    Some(digit) => {
                        second_digit = digit;
                        break;
                    }
                    _ => (),
                }
            }
        }
        calibration_sum += (first_digit as u64) * 10 + (second_digit as u64);
    }
    println!("{}", calibration_sum);
}