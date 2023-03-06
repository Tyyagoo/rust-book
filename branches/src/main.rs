fn main() {
    let number = 3;

    if number < 6 {
        println!("under average :(");
    } else {
        println!("above average ;)");
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter > 10 {
            break counter * 2;
        }
    };

    println!("The result is: {result}");

    let arr = [1, 2, 3, 4, 5];
    let mut acc = 0;

    for x in arr {
        acc += x;
    }

    println!("The acc is: {acc}");
}
