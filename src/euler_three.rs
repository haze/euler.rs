


fn largest_prime_factor(x: i64) -> i64 {
    let mut z = x;
    let mut d = 2;
    let mut factors = Vec::new();
    while z > 1 {
        while z % d == 0 {
            factors.push(d);
            z = z / d;
        }
        d += 1;
    }
    *factors.last().expect("no lcf found")
}

fn main() {
    let lcf = 600851475143;
    let res: i64 = largest_prime_factor(lcf);
    println!("lcf {lcf} => {res}", lcf=lcf, res=res);
}