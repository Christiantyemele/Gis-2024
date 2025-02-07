fn main() {
    println!("{}", fac(6));
}
fn fac(mut a: i32) -> i32 {
    let mut val = a - 1;
    let mut result = 0;
    if a == 0 {
        result = 1
    }
    while val >= 1 {
        result = a * val;
        a = result;
        val = val - 1;
    }
    result
}