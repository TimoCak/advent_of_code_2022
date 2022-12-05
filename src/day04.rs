use std::io::BufReader;
use std::{fs::File, io::BufRead};

pub fn analyze_data() {
    let file = File::open("day04.txt").unwrap();
    let file2 = File::open("day04.txt").unwrap();

    let reader = BufReader::new(file);
    let reader2 = BufReader::new(file2);

    let data_iterator = reader.lines();
    let overlap_iterator = reader2.lines();

    let mut sum = 0;
    let mut overlap_sum = 0;

    for i in data_iterator {
        if contain_pair(anaylze_line(match i {
            Ok(value) => value,
            Err(e) => e.to_string(),
        })) {
            sum += 1;
        }
    }

    for i in overlap_iterator {
        if overlap(anaylze_line(match i {
            Ok(value) => value,
            Err(e) => e.to_string(),
        })) {
            overlap_sum += 1;
        }
    }

    println!("SUMME: {}", sum);
    println!("SUMME: {}", overlap_sum);
}

fn anaylze_line(line: String) -> [usize; 4] {
    let mut first_pair = String::new();
    let mut second_pair = String::new();

    let mut is_second = false;

    let mut first_begin = String::new();
    let mut first_end = String::new();
    let mut second_begin = String::new();
    let mut second_end = String::new();

    //first_pair/second_pair
    for i in line.chars() {
        if is_second {
            second_pair.push(i);
        } else if i != ',' {
            first_pair.push(i);
        }
        if i == ',' {
            is_second = true;
        }
    }
    is_second = false;
    //first_begin/first_end
    for i in first_pair.chars() {
        if i == '-' {
            is_second = true;
        }
        if is_second && i != '-' {
            first_end.push(i);
        } else if i != '-' {
            first_begin.push(i);
        }
    }
    is_second = false;
    for i in second_pair.chars() {
        if i == '-' {
            is_second = true;
        }
        if is_second && i != '-' {
            second_end.push(i);
        } else if i != '-' {
            second_begin.push(i);
        }
    }

    let pair_arr = [
        first_begin.parse::<usize>().unwrap(),
        first_end.parse::<usize>().unwrap(),
        second_begin.parse::<usize>().unwrap(),
        second_end.parse::<usize>().unwrap(),
    ];

    pair_arr
}

fn contain_pair(pair_arr: [usize; 4]) -> bool {
    if (pair_arr[0] >= pair_arr[2] && pair_arr[1] <= pair_arr[3])
        || (pair_arr[0] <= pair_arr[2] && pair_arr[1] >= pair_arr[3])
    {
        return true;
    }
    false
}

fn overlap(pair_arr: [usize; 4]) -> bool {
   if ((pair_arr[0] >= pair_arr[2]) && (pair_arr[0] <= pair_arr[3])) 
   || ((pair_arr[1] >= pair_arr[2]) && (pair_arr[1] <= pair_arr[3]))
    {
    return true;
   }
   if ((pair_arr[2] >= pair_arr[0]) && (pair_arr[2] <= pair_arr[1])) 
   || ((pair_arr[3] >= pair_arr[0]) && (pair_arr[3] <= pair_arr[1]))
    {
    return true;
   }
    false
}
