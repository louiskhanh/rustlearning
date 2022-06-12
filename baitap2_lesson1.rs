use std::io::stdin;

fn count_str(slice: &str,userInput:String) -> i32 {
    let count:i32;
    let mut result = 0;

    count = slice.matches(&userInput.trim()).count() as i32;
    return count;
}

fn main() {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string)
         .ok()
         .expect("Failed to read line");

    let slice = "This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.";
    
    let count:i32;

    count = count_str(slice, input_string);
    println!("Number of occurrences: {}",count); 
}