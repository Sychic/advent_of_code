#[allow(dead_code)]
pub mod part1 {
    use std::fs;

    use itertools::Itertools;

    pub fn main() {
        let text = read_file();
        let data = text.split("\n").map(parse_int).collect_vec();
        let changes = to_changes(data);
        let (increments, decrements) = count_changes(changes);
        println!("There are {:?} increments and {:?} decrements", increments, decrements)
    }
    
    pub(super) fn read_file() -> String {
        fs::read_to_string("day1.txt")
        .expect("uh oh, something broke!")
    }
    
    pub(super) fn parse_int(string: &str) -> i32 {
        let res = string.trim().parse::<i32>();
        if res.is_err() {
            println!("parse error: {string}");
        }
        res.unwrap()
    }
    
    pub(super) enum Change {
        Increase,
        Decrease
    }
    
    pub(super) fn to_changes(list: Vec<i32>) -> Vec<Change> {
        let window_iter = list.as_slice().windows(2);
        let mut res: Vec<Change> = Vec::new();
        for window in window_iter {
            let change = if window[1] > window[0] { Change::Increase } else { Change::Decrease };
            res.push(change)
        }
        res
    }
    
    pub(super) fn count_changes(changes: Vec<Change>) -> (i32, i32) {
        changes.iter()
            .fold(
                (0, 0), 
                |acc, change| {
                    let mut step = acc;
                    if matches!(change, Change::Increase) {
                        step.0 += 1;
                    } else {
                        step.1 += 1;
                    }
                    step
                }
            )
    }
}

#[allow(dead_code)]
pub mod part2 {
    use itertools::Itertools;

    use crate::day1::part1;

    pub fn main() {
        let text = part1::read_file();
        let data = text.split("\n");
        let summed_data = sum_windows(data.collect_vec(), 3);
        let changes = part1::to_changes(summed_data);
        let (increments, decrements) = part1::count_changes(changes);
        println!("There are {:?} increments and {:?} decrements", increments, decrements)
    }

    fn sum_arr(array: &[i32]) -> i32 {
        let mut res = 0;
        for ele in array {
            res += ele;
        }
        res
    }

    fn sum_windows(data: Vec<&str>, window_size: usize) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let windows = data.windows(window_size);
        for ele in windows {
            let mut sum = 0;
            for string in ele {
                sum += part1::parse_int(*string);
            }
            res.push(sum);
        }
        res
    }
}