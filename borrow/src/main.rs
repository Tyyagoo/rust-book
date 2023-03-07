fn main() {
    let x = String::from("Hello");
    let y = String::from("world");
    greet(&x, &y);
    println!("It's me again! {} {}", x, y);

    deref();
    shared_ref();
    unique_ref();
    unique_ref2();
}

fn greet(x: &String, y: &String) -> () {
    println!("{} {}!", x, y);
}

fn deref() -> () {
    let mut x: Box<i32> = Box::new(1);
    let xv = *x;
    assert_eq!(xv, 1);

    *x += 1;
    let xv = *x;
    assert_eq!(xv, 2);

    // ==============================
    let y: &Box<i32> = &x;
    let yv = **y;
    assert_eq!(yv, 2);

    // ==============================
    let z: &i32 = &*x;
    let zv = *z;
    assert_eq!(zv, 2);
}

fn shared_ref() -> () {
    let mut vec: Vec<i32> = vec![0, 1, 2];
    // vec -> +R +W +O
    let x: &i32 = &vec[1];
    // vec ->  R -W -O
    // x   -> +R -- +O
    // *x  -> +R -- +O
    println!("The second elem of vec is: {}", *x);
    // vec ->  R +W +O
    // x   -> -R -- -O
    // *x  -> -R -- -O
    vec.push(4);
    // vec -> -R -W O
}

fn unique_ref() -> () {
    let mut vec: Vec<i32> = vec![1, 2, 3];
    // vec -> +R +W +O
    let x: &mut i32 = &mut vec[1];
    // vec -> -R -W -O
    // x   -> +R -- +O
    // *x  -> +R +W +O
    println!("The value of vec[1] is: {}", *x);
    *x += 2;
    println!("But now it' is: {}", *x);
    // vec -> +R +W +O
    // x   -> -R -- -O
    // *x  -> -R -W -O
    vec.push(5);
    println!("Vector: {:?}", vec);
    // vec -> -R -W -O
}

fn unique_ref2() -> () {
    let mut vec: Vec<i32> = vec![1, 2, 3];
    let x: &mut i32 = &mut vec[1];
    let y: &i32 = &*x;

    println!("{} {}", x, y);
}
