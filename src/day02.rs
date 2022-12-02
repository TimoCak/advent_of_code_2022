use crate::day01;


pub fn analyze_data() {
    println!("{}", calc_total_points(save_my_shapes_in_vec(), save_opponent_shapes_in_vec()));
}

fn determine_winning_points_per_round (opponent_shape: String, my_shape: String) -> u32 {
    //Outcome winning/losing!!
    if opponent_shape == "A" && my_shape == "Y"  {
        return 2 + 6;
    }
    if opponent_shape == "A" && my_shape == "Z" {
        return 3 + 0; 
    }
    if opponent_shape == "B" && my_shape == "X" {
        return 1 + 0; 
    }
    if opponent_shape == "B" && my_shape == "Z"  {
        return 3 + 6; 
    }
    if opponent_shape == "C" && my_shape == "X"  {
        return 1 + 6; 
    }
    if opponent_shape == "C" && my_shape == "Y"  {
        return 2 + 0; 
    }
    //draw!
    if opponent_shape == "A" && my_shape== "X" {
        return 1 + 3;
    }
    if opponent_shape == "B" && my_shape== "Y" {
        return 2 + 3;
    }
    if opponent_shape == "C" && my_shape== "Z" {
        return 3 + 3;
    }
    0
}

fn save_my_shapes_in_vec() -> Vec<String> {
    let text = match day01::get_file("day02.txt") {
        Ok(file) => file,
        Err(e) => e.to_string(),
    };

    let iterator_string = text.chars();
    let mut my_shape_vec: Vec<String> = Vec::new();

    for i in iterator_string {  
        if i.to_string() == "X" ||  i.to_string() == "Y" || i.to_string() == "Z"{
            my_shape_vec.push(i.to_string());
        }
        
    }
    my_shape_vec
}

fn save_opponent_shapes_in_vec() -> Vec<String> {
    let text = match day01::get_file("day02.txt") {
        Ok(file) => file,
        Err(e) => e.to_string(),
    };

    let iterator_string = text.chars();
    let mut opponent_shape_vec: Vec<String> = Vec::new();

    for i in iterator_string {  
        if i.to_string() == "A" ||  i.to_string() == "B" || i.to_string() == "C"{
            opponent_shape_vec.push(i.to_string());
        }
        
    }
    opponent_shape_vec
}


fn calc_total_points(vec_my_shape: Vec<String>, vec_opponent_shape: Vec<String>) -> u32 {
    let mut sum = 0;
    for i in 0..vec_my_shape.len() {
        sum += determine_winning_points_per_round(
             match vec_opponent_shape.get(i) {
                Some(value) => value.to_string(),
                None => String::from("_"),
             },
             match vec_my_shape.get(i) {
                Some(value) => value.to_string(),
                None => String::from("_"),
             });
    }
    sum
}
