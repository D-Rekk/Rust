use std::io;
use std::io::{stdout, Write};
#[allow(unused_parens)]
fn main() {
    /* 
    Euclidean Algorithm: take two number (a & b) both > 0
    if (b > a) swap them. then divide a/b. The remainder is c.
    while (c != 0) a = b; b = c; keep dividing a/b
    */

    let mut a= String::new();
    let mut b = String::new();

    print!("> Enter your 1* number: ");
    stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut a)
        .expect("Some Error brr")
        .to_string()
    ;

    print!("> Enter your 2* number: ");
    stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut b)
        .expect("Some Error brr")
        .to_string()
    ;
    
    let mut a:u32 = a.trim().parse().expect("Couldn't convert to string");
    let mut b:u32 = b.trim().parse().expect("Couldn't convert to string");
    assert!(a != 0 || b!=0, "Number is 0");
    while  b != 0{
        if (a > b) {
            let t = b;
            b = a;
            a = t;
        }
        a = a%b
    }
    println!("The gread common divisor is: {a}");

}
