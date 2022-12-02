use std::fs;

#[allow(dead_code)]
pub enum InputType {
    Real,
    Example,
}

pub fn input(day: u8, input_type: InputType) -> String {
    match input_type {
        InputType::Real => read_input(day),
        InputType::Example => read_example_input(day),
    }
}

fn read_input(day: u8) -> String {
    match fs::read_to_string(format!("input/day{:0>2}.txt", day)) {
        Ok(result) => result,
        Err(err) => panic!("Error: {}", err),
    }
}

#[allow(dead_code)]
fn read_example_input(day: u8) -> String {
    match fs::read_to_string(format!("input/day{:0>2}-example.txt", day)) {
        Ok(result) => result,
        Err(err) => panic!("Error: {}", err),
    }
}
