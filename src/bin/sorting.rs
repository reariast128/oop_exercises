use std::{u16, vec};

fn read_user_vector_length() -> u16 {
    println!("Introduce la cantidad de elementos que va a tener el vector: ");

    loop {
        let mut user_vector_length = String::new();
        std::io::stdin().read_line(&mut user_vector_length);
        match user_vector_length.trim().parse::<u16>() {
            Ok(num) => {
                return num;
            }
            Err(_) => {
                println!("No introduciste un entero correcto. Vuelve a escribirlo.");
            },
        }
    }
}

fn read_user_numbers(vector_size: u16) -> Vec<i128> {
    let mut numbers_vector: Vec<i128> = Vec::new();
    for number_index in 0..vector_size {
        println!("Introduce el número {} de {}", number_index, vector_size);

        let mut user_number = String::new();
        std::io::stdin().read_line(&mut user_number);

        match user_number.trim().parse::<i128>() {
            Ok(num) => numbers_vector.push(num),
            Err(_) => println!("No introduciste un entero correcto. Se ignorará este elemento.")
        }
    }
    return numbers_vector;
}

fn sort_vector (number_vector: Vec<i128>) -> Vec<i128>{
    
    let mut sorting_vector: Vec<i128> = number_vector;

    for _ in 1..sorting_vector.len() {
        for internal in 0..sorting_vector.len() - 1 {
            if sorting_vector[internal] > sorting_vector[internal + 1] {
                let swapper: i128 = sorting_vector[internal];
                sorting_vector[internal] = sorting_vector[internal + 1];
                sorting_vector[internal + 1] = swapper;
            }
        }
    }
    return sorting_vector;
}

fn main() {
    let vector_length = read_user_vector_length();
    let mut number_vector = read_user_numbers(vector_length);
    number_vector = sort_vector(number_vector);
    
    println!("El vector ordenado es:\n");

    for num in number_vector {
        println!("{}", num);
    }

}
