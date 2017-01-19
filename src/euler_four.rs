


fn palindrome(x: i64) -> ((i64, i64), bool) {
    let mut strx: String = x.to_string();
    let len: usize = strx.len();
    if len % 2 == 0 {
        let l_half: String = strx.drain(.. len / 2).rev().collect();
        ((l_half.parse::<i64>().unwrap_or(-1), strx.parse::<i64>().unwrap_or(-1)), l_half == strx)
    } else {
        let half = (len as f64) / 2f64;
        let half_floor = half.floor() as usize;
        let rev_drain: String = strx.drain(half_floor + 1 ..).rev().collect();
        let drain: String = strx.drain(.. half_floor).collect();
        ((rev_drain.parse::<i64>().unwrap_or(-1), drain.parse::<i64>().unwrap_or(-1)), rev_drain == drain)
    }
}


fn main() {
    let mut top: Option<(i64, (i64, i64), bool)> = Option::None;
    let min: i64 = 100;
    let max: i64 = 999;
    for x in min..max {
        for y in min..max {
            let product = x * y;
            let (temptup, fits) = palindrome(product);
            let (lt, rt) = temptup;
            let (x_top, _, _) = top.unwrap_or((0, (0, 0), false));
            if (top.is_some() && fits && x_top != 0 && (lt * rt) > x_top) || (top.is_none() && fits) { 
                top = Option::Some((product, temptup, fits)); 
            }
        }
    }
    match top {
        Some((product, tup, _)) => println!("[{:?}] => {:?}", product, tup),
        None => println!("none found.")
    }
}