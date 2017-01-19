
fn fibonacci(until: i64) -> Vec<i64> {
    let mut start = vec![1, 2];
    while start.last().unwrap() < &until {
        let start_len: usize = start.len();
        let start_tail: i64 = *start.last().unwrap();
        let start_posterior: i64 = start[start_len - 2];
        start.push(start_posterior + start_tail);
    }
    start
}

fn main() {
    let is_even = |x: &i64| *x % 2 == 0;
    let mut fibs: Vec<i64> = fibonacci(4_000_000);
        fibs.retain(&is_even);
        
    let sum: i64 = fibs.into_iter().sum();
    println!("sum => {:?}", sum);
}