use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file(file_path: &str) -> Vec<(i32, i32)> {
    // Open the file
    let file = File::open(file_path).unwrap();
    
    // Create a buffered reader
    let reader = BufReader::new(file);
    
    // Create a vector to store the parsed numbers
    let mut numbers = Vec::new();
    
    // Read each line and parse it as two integers
    for line in reader.lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 {
                if let (Ok(num1), Ok(num2)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                    // Add the parsed numbers as a tuple to the vector
                    numbers.push((num1, num2));
                } else {
                    println!("Failed to parse numbers: {}", line);
                }
            } else {
                println!("Line does not contain exactly two numbers: {}", line);
            }
        } else {
            println!("Failed to read line");
        }
    }
    // Return the vector of tuples
    numbers
}


fn find_root(numbers: &mut Vec<i32>, mut p: i32) -> i32 {
    while p != numbers[p as usize] {
        p = numbers[p as usize];
    }
    p
}

fn quick_union(numbers: &mut Vec<i32>, p: i32, q: i32) {
    let root_p = find_root(numbers, p);
    let root_q = find_root(numbers, q);
    // same root, already connected
    if root_p == root_q {
        return;
    }
    numbers[root_p as usize] = root_q;
}

fn connected(numbers: &mut Vec<i32>, p: i32, q: i32) -> bool {
    let res = find_root(numbers, p) == find_root(numbers, q);
    res
}


fn main() {
    
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide the file input path as the first argument");
        return;
    }
    let file_path = &args[1];
    
    // couples of integers are stored in a vector, one tuple per index
    let numbers = read_file(file_path);

    let max_value = numbers.iter()
                                .flat_map(|&(a, b)| vec![a, b])
                                .max()
                                .unwrap_or(0) + 1;
    // Store the connection, each index is a number, the value is the root
    // When index and value are the same, it means the number is a root of a tree
    let mut id: Vec<i32> = (0..max_value).collect();

    // For Weighted Quick Union, we need to store the size of the tree
    let mut sz: Vec<i32> = vec![0; max_value as usize];

    print!("length of numbers: {}, max value: {}\n", numbers.len(), max_value -1 );
    
    for (i, val ) in numbers.iter().enumerate() {
        println!("index {}, 1st: {}, 2nd: {}", i, val.0, val.1);
        if !connected(&mut id, val.0, val.1) {
            quick_union(&mut id, val.0, val.1);
            println!("Connected {} and {}\nidx: {:?}\nid:  {:?}", val.0, val.1, (0..max_value).collect::<Vec<_>>(), id);
        }
    }
    print!("Final id vector: \nidx: {:?}\nid:  {:?}\nsz:  {:?}", (0..max_value).collect::<Vec<_>>(), id, sz);
}