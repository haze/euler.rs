
fn multiples(of: i64, until: i64) -> Vec<i64> {
    let mut mults = vec![of];
    let mut count = 2;
    while mults.last().unwrap() < &until && (count * of) < until {
        mults.push(of * count);
        count += 1;
    }
    mults
}

fn main() {
    let mut threes = multiples(3, 1000);
    let mut fives = multiples(5, 1000);

    threes.append(&mut fives);

    threes.sort();
    threes.dedup();

    let sum: i64 = threes.into_iter().sum();

    println!("sum = {:?}", sum);
}
