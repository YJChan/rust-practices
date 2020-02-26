use std::io::stdin;
use rand::Rng;

fn main() {
    let input = user_input();
    let v: Vec<u8> = gen_random(input);

    print_arr(&v);

    let b_sorted = bubble_sort(&v);

    print_arr(&b_sorted);
}

fn bubble_sort(v: &Vec<u8>) -> Vec<u8> {
    let mut temp;
    let mut target = v.clone();
    let len = v.len();

    for _ in 0..len {
        let mut done = true;
        for j in 1..len {
            if compare_2_val(target[j - 1], target[j]) {
                temp = target[j - 1];
                target[j - 1] = target[j];
                target[j] = temp;
                done = false;
            }
        }
        if done {
            break;
        }
    }

    target
}

fn compare_2_val(num1: u8, num2: u8) -> bool {
    if num1 > num2 {
        true
    } else {
        false
    }
}

fn user_input() -> u8 {
    let max_loop = 10;

    let mut str_input = String::new();
    println!("Enter number to generate for sorting: min = 1, max = 10");
    
    stdin().read_line(&mut str_input).expect("Only numeric number is allowed");

    let mut input:u8 = str_input.trim().parse().unwrap();

    println!("input is {}", input);

    if input > max_loop {
        println!("Number generate to sort not greater than 10");
        input = user_input();
    }

    input
}

fn gen_random(num: u8) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::new();
    let mut rng = rand::thread_rng();
    let mut counter = 0;

    loop {
        v.push(rng.gen());
        counter += 1;

        if counter == num {
            break;
        }
    }

    v
}

fn print_arr(v: &Vec<u8>) {
    for i in v {
        print!("{}, ", i);
    }
    println!("");
}