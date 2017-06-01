

fn is_prime(n: u32) -> bool {
	(2..n).all(|x| n % x != 0)
}

/* Benchmarks
	v1: Using a vec to count with len: 25 seconds
	v2: Ditching the vec and using a counter variable 25 seconds ? ??? ?
*/

// suprising how we don't have an algorithm for this yet.
fn nth_prime(n: u32) -> u32 {
	fn next_prime(n: u32) -> u32 { if is_prime(n) { n } else { next_prime(n + 1) } }
	(1..n).fold(2, |last, _| next_prime(last + 1) )
}

fn main() {
	let n = 10001;
	println!("{}th prime = {}", n, nth_prime(n));
}