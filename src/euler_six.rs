
fn sum_squares(from: i64, to: i64) -> i64 {
    (from..(to+1)).map(|x| 
    (x as f64).powi(2) as i64).sum()
}

fn sum_range(from: i64, to: i64) -> i64 {
    (from..(to+1)).sum()
}

fn main() {
    let sum_of_squares = sum_squares(1, 100);
    let square_of_sum = (sum_range(1, 100) as f64).powi(2) as i64;
    println!(" {:?} <|> {:?} [[ diff = {:?} ]]", square_of_sum, sum_of_squares, square_of_sum - sum_of_squares);
}  