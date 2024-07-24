use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file(file_path: &str) -> Vec<i32> {
    // Open the file
    let file = File::open(file_path).unwrap();
    
    // Create a buffered reader
    let reader = BufReader::new(file);
    
    // Create a vector to store the parsed numbers
    let mut numbers = Vec::new();
    
    // Read each line and parse it as an integer
    for line in reader.lines() {
        if let Ok(number) = line {
            if let Ok(parsed_number) = number.parse::<i32>() {
                // Add the parsed number to the vector
                numbers.push(parsed_number);
            } else {
                println!("Failed to parse number: {}", number);
            }
        } else {
            println!("Failed to read line");
        }
    }
    // Return the vector of numbers
    numbers
}


// fn quick_union(numbers: &mut Vec<i32>, p: i32, q: i32) {
//     let _root_p = find_root(numbers, p);
//     let _root_q = find_root(numbers, q);
//     numbers[root_p] = root_q;
// }

// fn find_root(numbers: &mut Vec<i32>, mut p: i32) -> i32 {
//     while p != numbers[p] {
//         p = numbers[p];
//     }
//     p
// }


fn main() {
    
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide the file input path as the first argument");
        return;
    }
    let file_path = &args[1];
    
    // Call the read_file function with the file path
    let numbers = read_file(file_path);

    let tree: Vec<i32> = Vec::from_iter(0..numbers.len() as i32);

    print!("length of numbers: {}\n", numbers.len());
    
    for (i, val ) in numbers.iter().enumerate() {
        println!("{} {} | tree {}", i, val, tree[i]);
    }
}