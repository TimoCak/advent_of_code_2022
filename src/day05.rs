use std::vec;
use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn analyze_data() {

    //stacks
    let mut stack1: Vec<char> = vec!['Q', 'F', 'M', 'R', 'L', 'W', 'C', 'V'];
    let mut stack2: Vec<char> = vec!['D', 'Q', 'L'];
    let mut stack3: Vec<char> = vec!['P', 'S', 'R', 'G', 'W', 'C', 'N', 'B'];
    let mut stack4: Vec<char> = vec!['L', 'C', 'D', 'H', 'B', 'Q', 'G'];
    let mut stack5: Vec<char> = vec!['V', 'G', 'L', 'F', 'Z', 'S'];
    let mut stack6: Vec<char> = vec!['D', 'G', 'N', 'P'];
    let mut stack7: Vec<char> = vec!['D', 'Z', 'P', 'V', 'F', 'C', 'W'];
    let mut stack8: Vec<char> = vec!['C', 'P', 'D', 'M', 'S'];
    let mut stack9: Vec<char> = vec!['Z', 'N', 'W', 'T', 'V', 'M', 'P', 'C'];

    //stack-array
    let stack_vec = [stack1, stack2, stack3, stack4, stack5, stack6, stack7, stack8, stack9];
    
    read_and_return_commands();
}

fn read_and_return_commands() {
    let file = File::open("day05.txt").unwrap();
    let reader = BufReader::new(file);

    let line_iterator = reader.lines();

    

    for i in line_iterator {
        analyze_command_string(match i {
            Ok(value) => value,
            Err(e) => e.to_string(),
        });
    }

    
}

fn analyze_command_string(line: String) -> Vec<[u32;3]>{
    let mut command_vec = Vec::new();
    let mut command_arry = [0;3];
    let mut arry_counter = 0;

    for i in line.chars() {
        if arry_counter==command_arry.len() {
            arry_counter = 0;
            println!("{:?}",command_arry);
            command_vec.push(command_arry);
            command_arry = [0;3];
        }
        if i.is_numeric() {
            command_arry[arry_counter] = i.to_digit(10).unwrap();
            arry_counter += 1;
        }
    }
    command_vec
}
