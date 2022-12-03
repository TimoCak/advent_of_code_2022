// Lowercase item types a-z have priority 1-26
// Uppercase items type A-Z have 27-52
use crate::day01;

fn store_all_first_compartments() -> Vec<String>  {
    let text = match day01::get_file("day03.txt") {
        Ok(file) => file,
        Err(e) => e.to_string(),
    };

    let iterator_char = text.chars();
    let mut vec_compartments1: Vec<String> = Vec::new();
    let mut save_string: String = String::new();

    for i in iterator_char {
        save_string.push(i);

        if i.to_string() == "\n" {
            let mut compartment1 = String::new();
            for j in 0..(save_string.len()/2)-1  {
                compartment1.push(save_string.as_bytes()[j] as char); 
            }
            vec_compartments1.push(compartment1);
            save_string = String::new();
        }
    }
    vec_compartments1
} 

fn store_all_second_compartments() -> Vec<String>  {
    let text = match day01::get_file("day03.txt") {
        Ok(file) => file,
        Err(e) => e.to_string(),
    };

    let iterator_char = text.chars();
    let mut vec_compartments2: Vec<String> = Vec::new();
    let mut save_string: String = String::new();

    for i in iterator_char {
        save_string.push(i);
      
        if i.to_string() == "\n" {
            let mut compartment2 = String::new();
            for j in (save_string.len()/2)-1..save_string.len()-2 {
                if save_string.as_bytes()[j] as char != '\n' && save_string.as_bytes()[j] as char != '\r'{
                    compartment2.push(save_string.as_bytes()[j] as char);
                }
            }
            vec_compartments2.push(compartment2);
            save_string = String::new();
        }
    }
    vec_compartments2
}


fn compare_string_vecs(compartment1: String, compartment2: String) -> String {
    let mut common_items: String = String::new();

    let vector_length = compartment1.len();
    let vector_length2: usize = compartment2.len();
    let compartment1_bytes = compartment1.as_bytes();
    let compartment2_bytes = compartment2.as_bytes();


    for i in 0..vector_length {
        for j in 0..vector_length2 {
            if (compartment1_bytes[i] as char).to_string() == (compartment2_bytes[j] as char).to_string() {
                common_items.push(compartment2.as_bytes()[j] as char);
            } 
        }
        
    }
    common_items
}

fn validate_points(letter: String) -> usize {
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    for i in 0..alphabet.len() {
        if (alphabet.as_bytes()[i] as char).to_string() == letter {
            println!("ITEM: {:?}",letter);
            println!("PRIORITY: {}",i+1);
            return i+1;
        }
    }
    0
}

pub fn analyze_data() {
    //go through 2 vecs and compare
    let mut sum = 0;

    let mut common_letters: Vec<String> = Vec::new();
    for i in 0..store_all_first_compartments().len() {
            common_letters.push(compare_string_vecs(match store_all_first_compartments().get(i) {
            Some(value) => value.to_string(),
            None => String::from(""),
        }, 
        match store_all_second_compartments().get(i){
            Some(value) => value.to_string(),
            None => String::from(""),
        }));
    }
    
    for i in 0..common_letters.len() {
        sum += validate_points(match common_letters.get(i){
            Some(value) => value.to_string(),
            None => String::from(""),
        });
        println!("{:?}", common_letters.get(i));
    }
    println!("{}", sum);
    
}