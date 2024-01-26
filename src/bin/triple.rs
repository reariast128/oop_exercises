fn main() {
    let mut array: Vec<i32> = Vec::new();

    for middle in 1..3 {
        for right in 1..3 {
            for left in 1..9 {
                array.push(left*100+middle*10+right)
            }
            
        }
    }

    for triple in array {
        println!("{}", triple);
    }

}
