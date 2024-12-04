use regex::Regex;

use crate::utils::read_input;


pub fn solve() {
    let input = read_input(3);
    println!("{}", process_a(&input));
    println!("{}", process_b(&input));
}

fn prod(x: i32, y: i32) -> i32 {
    x * y
}

fn process_a(_input: &str) -> i32 {
    let mut s = 0;
    let re = Regex::new(r"mul\((?<x>[0-9]{1,3}),(?<y>[0-9]{1,3})\)").unwrap();
    let matches: Vec<(i32, i32)> = re.captures_iter(_input).map(|caps| {
        let x = caps.name("x").unwrap().as_str().parse::<i32>().unwrap();
        let y = caps.name("y").unwrap().as_str().parse::<i32>().unwrap();
        (x,y)
    }).collect();

    for (x,y) in matches {
        s += prod(x,y)
    }
    s
}

fn process_b(_input: &str) -> i32 {
    let mut s = 0; 
    let mut i = 0;
    let mut enabled = true;
    let re = Regex::new(r"^mul\((\d+),(\d+)\)").unwrap();
    
    while i < _input.len() {
        if _input[i..].starts_with("do()") {
            enabled = true;
            i += 4 

        } else if _input[i..].starts_with("don't()") {
            enabled = false;
            i += 7

        } else if let Some(caps) = re.captures(&_input[i..]) {
            let x = caps[1].parse::<i32>().unwrap();
            let y = caps[2].parse::<i32>().unwrap();
            if enabled {
                s += prod(x,y);
            }
            i += caps[0].len(); 
            
        } else {
            i += 1;
        }
    }

    s
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_file;

    #[test]
    fn test_a() {
        let example = read_file(3, "example");
        assert_eq!(process_a(&example), 161);
    }
    
    #[test]
    fn test_b() {
        let example = read_file(3, "example2");
        assert_eq!(process_b(&example), 48);
    }

}