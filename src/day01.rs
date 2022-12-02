use std::fs::File;
use std::io;
use std::io::Read;

pub fn get_file() -> Result<String, io::Error> {
    let mut text = String::new();

    File::open("day01.txt")?.read_to_string(&mut text)?;

    Ok(text)
}

pub fn search_for_max(v: &mut Vec<u32>) -> Option<&u32> {
    let mut max = v.get(0);
    for i in 0..v.len()  {
        if max < v.get(i) {
            max = v.get(i);
            println!("{:?}",max);
        }
    }
    match max {
        Some(max) => Some(max),
        None => None,
    }
}

