fn main() {
    let c = 37.0;
    let f = 100.0;

    println!("{c}ºC in ºF is: {}ºF", c_to_f(c));
    println!("{f}ºF in ºC is: {}ºC", f_to_c(f));

    if c_to_f(f_to_c(100.0)) != 100.0 {
        panic!();
    }
}

fn f_to_c(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn c_to_f(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}
