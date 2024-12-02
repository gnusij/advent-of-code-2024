use crate::utils::read_input;

pub fn solve() {
    let input = read_input(2);
    println!("{}", process_a(&input));
    println!("{}", process_b(&input));
}

fn process_input(_input: &str) -> Vec<Vec<i32>> {
    let mut vec: Vec<Vec<i32>> = Vec::new();

    for line in _input.lines() {
        let mut tmp_vec = Vec::new();
        let parts: Vec<&str> = line.split_whitespace().collect();
        for part in parts {
            if let Ok(a) = part.parse::<i32>() {
                tmp_vec.push(a);
            }
        }
        vec.push(tmp_vec);
    }
    vec
}

fn is_same_sign(arr: &Vec<i32>) -> bool {
    for i in 1..arr.len() {
        if arr[i] == 0 {
            continue;
        }
        if arr[i].signum() != arr[0].signum() {
            return false;
        }
    }
    true
}

fn is_within_one2three(arr: &Vec<i32>) -> bool {
    for item in arr {
        if item.abs() < 1 || item.abs() > 3{
            return false;
        }
    }
    true
}

fn is_safe(arr: &Vec<i32>) -> bool {
    
    let mut tmp_vec = Vec::new();
    for i in 1..arr.len() {
        tmp_vec.push(arr[i]-arr[i-1]);
    }
    if is_same_sign(&tmp_vec) && is_within_one2three(&tmp_vec) {
        return true;
    }
    false
}

fn process_a(_input: &str) -> i32 {
    let mut s = 0;
    for line in &process_input(_input) {
        if is_safe(line) {
            s += 1;
        }
    }
    s
}

fn process_b(_input: &str) -> i32 {
    let mut s = 0;
    for line in &process_input(_input) {
        if is_safe(line) {
            s += 1;
        }
        else {
            for i in 0..line.len() {
                let mut tmp = line.clone();
                tmp.remove(i);
                if is_safe(&tmp) {
                    s += 1;
                    break;
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
        let example = read_example(2);
        assert_eq!(process_a(&example), 2);
    }
    
    #[test]
    fn test_b() {
        let example = read_example(2);
        assert_eq!(process_b(&example), 4);
    }

    #[test]
    fn test_is_same_sign() {
        let arr = vec![1, 2, 3, 4];
        assert_eq!(is_same_sign(&arr), true);
        let arr = vec![1, 2, 3, -4];
        assert_eq!(is_same_sign(&arr), false);
        let arr = vec![-1, 0, -3, -4];
        assert_eq!(is_same_sign(&arr), true);
    }

    #[test]
    fn test_is_within_one2three() {
        let arr = vec![1, 2, 3];
        assert_eq!(is_within_one2three(&arr), true);
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(is_within_one2three(&arr), false);
        let arr = vec![-1, 0, -3, -1];
        assert_eq!(is_within_one2three(&arr), false);
        let arr = vec![-1, 0, -3, -4, -5];
        assert_eq!(is_within_one2three(&arr), false);
    }

    #[test]
    fn test_is_safe() {
        let arr = vec![7, 6, 4, 2, 1];
        assert_eq!(is_safe(&arr), true);
        let arr = vec![1, 2, 7, 8, 9];
        assert_eq!(is_safe(&arr), false);
        let arr = vec![9, 7, 6, 2, 1];
        assert_eq!(is_safe(&arr), false);
        let arr = vec![1, 3, 2, 4, 5];
        assert_eq!(is_safe(&arr), false);
        let arr = vec![8, 6, 4, 4, 1];
        assert_eq!(is_safe(&arr), false);
        let arr = vec![1, 3, 6, 7, 9];
        assert_eq!(is_safe(&arr), true);
    }

}