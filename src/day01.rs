use std::fs::File;
use std::io;
use std::io::Read;

pub fn get_file(path :&str) -> Result<String, io::Error> {
    let mut text = String::new();

    File::open(path)?.read_to_string(&mut text)?;

    Ok(text)
}

pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for _i in 0..arr.len() {
        for item in 0..arr.len() {
            if item < arr.len() - 1 {
                if arr[item] > arr[item + 1] {
                    arr.swap(item, item + 1);
                }
            }
        }
    }
}

pub fn search_for_max(v: &mut Vec<u32>) -> Option<&u32> {
    bubble_sort(v);
    let mut max = v.get(0);
    for i in 0..v.len() {
        println!("{:?}", v.get(i));
        if max < v.get(i) {
            max = v.get(i);
            println!("{:?}", max);
        }
    }
    println!(
        "The top three have a total amount of: {}",
        v.get(v.len() - 1).unwrap() + v.get(v.len() - 2).unwrap() + v.get(v.len() - 3).unwrap()
    );
    match max {
        Some(max) => Some(max),
        None => None,
    }
}

pub fn analyze_data() {
    let text = match get_file("day01.txt") {
        Ok(file) => file,
        Err(e) => e.to_string(),
    };

    let iterator_string = text.chars();

    let mut sum = 0;
    let mut v: Vec<u32> = Vec::new();
    let mut save_string = String::new();
    let mut n_counter = 0;

    for i in iterator_string {
        if i.to_string() == "\n" || i.to_string() == "\r" {
            if i.to_string() == "\n" {
                n_counter += 1;

                sum += match save_string.parse::<u32>() {
                    Ok(value) => value,
                    Err(_) => 0,
                };
                save_string = String::new();

                if n_counter >= 2 {
                    v.push(sum);
                    sum = 0;
                }

                continue;
            }
            continue;
        } else {
            save_string.push(i);
            n_counter = 0;
        }
    }

    println!("MAXIMUM: {:?}", search_for_max(&mut v));
}
