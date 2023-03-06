fn main() {
    let x = sum(15, 10);
    println!("The value of x is: {x}");
    print_labeled_measure("ºC", 26.7);
}

fn sum(x: i64, y: i64) -> i64 {
    x + y
}

fn print_labeled_measure(label: &str, measure: f64) {
    println!("The measurement is: {measure}{label}");
}
