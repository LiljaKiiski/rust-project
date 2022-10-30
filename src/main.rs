fn main() {
    println!("Hello, world!");
    let num = gcd(5, 10);

    println!("{}", num);
}

//u64 = unsigned 64 bit integer
//variable can't be changed unless it's mut

fn gcd(mut n: u64, mut m: u64)-> u64{
    assert!(n != 0 && m!=0);

    while m != 0 {
        if m < n {
            let t: u64 = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}