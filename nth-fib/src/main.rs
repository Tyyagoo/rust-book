fn main() {
    if nth(1) != 1 {
        panic!("nth(1) != 1");
    }
    if nth(2) != 1 {
        panic!("nth(2) != 1");
    }
    if nth(3) != 2 {
        panic!("nth(3) != 2");
    }
    if nth(4) != 3 {
        panic!("nth(4) != 3");
    }
    if nth(5) != 5 {
        panic!("nth(5) != 5");
    }
    if nth(6) != 8 {
        panic!("nth(6) != 8");
    }
    if nth(7) != 13 {
        panic!("nth(7) != 13");
    }
    if nth(8) != 21 {
        panic!("nth(8) != 21");
    }

    println!("The nth(11) is: {}", nth(11));
}

fn nth(n: u128) -> u128 {
    let (mut i, mut x, mut y) = (1, 1, 1);

    while i < n {
        let z = x + y;
        y = x;
        x = z;
        i += 1;
    }

    y
}
