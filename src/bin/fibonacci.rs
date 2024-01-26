fn main() {

    let mut b: i128 = 1;
    let mut a: i128 = 0;

    while b <= 10000{
        b = b + a;
        a = b - a;
    }

    println!("El resultado final es {}", b);
}