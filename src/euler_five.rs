


fn smallest_multiple() -> i64 {
    let max = 10_000_000_000; // JUST to be sure. ;)
    let min = 20;
    let mut highest = -1;
    for i in min .. max {
        let mut pass = true;
        for x in 1 .. 20 {
            pass = pass && (i % x == 0);
        }
        if pass {
            highest = i;
            break;
        }
    }
    highest
}

fn main() {
    println!("{:?}", smallest_multiple());
}