use crate::utils::read_input;


pub fn solve() {
    let input = read_input(4);
    println!("{}", process_a(&input));
    println!("{}", process_b(&input));
}

fn process_input(_input: &str) -> Vec<Vec<char>> {
    let mut vec: Vec<Vec<char>> = Vec::new();

    for line in _input.lines() {
        let mut tmp_vec = Vec::new();
        let parts: Vec<char> = line.chars().collect();
        for part in parts {
            tmp_vec.push(part);
        }
        vec.push(tmp_vec);
    }
    vec
}

fn get(x: i32, y: i32, _input: &Vec<Vec<char>>) -> Option<char> {
    if y >= 0 && y < _input.len() as i32 && x >= 0 && x < _input[y as usize].len() as i32 {
        Some(_input[y as usize][x as usize])
    } else {
        None
    }
}


fn is_match(x: i32, y: i32, _input: &Vec<Vec<char>>, direction: (i32, i32), matchstring: &str) -> bool {
    for (i, char) in matchstring.chars().enumerate() {
        let i = i as i32;
        if get(x + direction.0 * i, y + direction.1 * i, _input) != Some(char) {
            return false;
        }
    }
    true
}

fn process_a(_input: &str) -> i32 {
    let mut s = 0;
    let input = process_input(_input);
    let search_directions = [(1,0),(-1,0),(0,1),(0,-1),(1,1),(-1,-1),(1,-1),(-1,1)];

    for &direction in &search_directions {
        for y in 0..input.len() as i32 {
            for x in 0..input[0].len() as i32 {
                if is_match(x, y, &input, direction, "XMAS") {
                    s += 1;
                }
            }
        }
    }

    s
}

fn process_b(_input: &str) -> i32 {
    let mut s = 0; 
    let input = process_input(_input);

    for y in 0..input.len() as i32 {
        for x in 0..input[0].len() as i32 {
            if is_match(x, y, &input, (1,1), "MAS") || is_match(x, y, &input, (1,1), "SAM") {
                if is_match(x+2, y, &input, (-1,1), "MAS") || is_match(x+2, y, &input, (-1,1), "SAM") {
                    s += 1;
                }
            }
        }
    }
    s
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    #[test]
    fn test_a() {
        let example = read_example(4);
        assert_eq!(process_a(&example), 18);
    }
    
    #[test]
    fn test_b() {
        let example = read_example(4);
        assert_eq!(process_b(&example), 9);
    }

}