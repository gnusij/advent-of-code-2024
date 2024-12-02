use crate::utils::read_input;

pub fn solve() {
    let input = read_input(1);
    println!("{}", process_a(&input));
    println!("{}", process_b(&input));
}

fn process_input(_input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut vec_l = Vec::new();
    let mut vec_r = Vec::new();
    for line in _input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            if let (Ok(a),Ok(b)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                vec_l.push(a);
                vec_r.push(b);
            }
        }
    }   
    vec_l.sort();
    vec_r.sort();
    
    (vec_l, vec_r)
}

fn process_a(_input: &str) -> i32 {
    let mut s = 0;
    let (vec_l, vec_r) = process_input(_input);

    for (i, j) in vec_l.iter().zip(vec_r.iter()) {
        s += (*i - *j).abs();
    }

    s
}

fn process_b(_input: &str) -> i32 {
    let mut s = 0;
    let (vec_l, vec_r) = process_input(_input);

    for i in vec_l.iter() {
        let c = vec_r.iter().filter(|&n| *n == *i).count() as i32; 
        s += i * c;
    }

    s
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    #[test]
    fn test_a() {
        let example = read_example(1);
        assert_eq!(process_a(&example), 11);
    }
    
    #[test]
    fn test_b() {
        let example = read_example(1);
        assert_eq!(process_b(&example), 31);
    }
}