use std::fs ; 

fn main() {
    let input = fs::read_to_string("inputs/day1_input.txt")
        .expect("Failed to read input file") ;

    let(left_list, right_list) = parse_input(&input) ;

    let mut left_sorted = left_list.clone() ;
    let mut right_sorted = right_list.clone() ;
    left_sorted.sort() ;
    right_sorted.sort() ;

    let total_distance: i32 = left_sorted
        .iter()
        .zip(right_sorted.iter())
        .map(|(a, b)| (a - b).abs())
        .sum() ;


    println!("Totol distance between the lists: {}", total_distance) ;

}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left_list = Vec::new() ;
    let mut right_list = Vec::new() ;


    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect() ;
        if parts.len() == 2 {
            left_list.push(parts[0].parse::<i32>().expect("Invalid number in left list")) ;
            right_list.push(parts[1].parse::<i32>().expect("Invalid number in right list")) ;
        }
    }

    (left_list, right_list)
}