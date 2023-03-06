fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let y = 3;
    println!("The value of y is: {y}");
    let y = 5;
    println!("The value of y is: {y}");

    let z = 10;
    println!("The value of z is: {z}");

    {
        let z = "hello";
        println!("The value of z is: {z}");
    }

    println!("The value of z is: {z}");
}
