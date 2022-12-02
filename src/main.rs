mod day01;

fn main() {
    println!("Hello, world!");
    let text = match day01::get_file(){
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

                if n_counter>=2 {
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

    println!("MAXIMUM: {:?}",day01::search_for_max(&mut v));

}
