pub mod part1 {
    use std::fs;

    use itertools::Itertools;

    pub fn main() {
        println!("Please enter data: (end by typing `end`)");
        let text = read_file();
        let data = text.split("\n").map(parse_int).collect_vec();
        let changes = to_changes(data);
        let (increments, decrements) = count_changes(changes);
        println!("There are {:?} increments and {:?} decrements", increments, decrements)
    }
    
    fn read_file() -> String {
        fs::read_to_string("day1.txt")
        .expect("uh oh, something broke!")
    }
    
    fn parse_int(string: &str) -> i32 {
        let res = string.trim().parse::<i32>();
        if res.is_err() {
            println!("parse error: {string}");
        }
        res.unwrap()
    }
    
    enum Change {
        Increase,
        Decrease
    }
    
    fn to_changes(list: Vec<i32>) -> Vec<Change> {
        let window_iter = list.as_slice().windows(2);
        let mut res: Vec<Change> = Vec::new();
        for window in window_iter {
            let change = if window[1] > window[0] { Change::Increase } else { Change::Decrease };
            res.push(change)
        }
        res
    }
    
    fn count_changes(changes: Vec<Change>) -> (i32, i32) {
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