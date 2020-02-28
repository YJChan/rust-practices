use std::io::stdin;
use rand::Rng;
use std::time::{Instant};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::{self, BufRead};
use encoding::{Encoding, EncoderTrap, DecoderTrap};
use encoding::all::ASCII;
use std::io::BufWriter;
use std::str;

fn main() {
    // read_str_write_encoding();
    // write_buff();
    // read_encoded_txt();
    // read_buff();
    // encoding_decoding();
    // read_file();
    read_byte();
}

fn encoding_decoding() {
    let orig = "Hello, 世界!".to_string();
    let encoded = ASCII.encode(&orig, EncoderTrap::Ignore).unwrap();
    println!("{:?}", &encoded);

    let decoded = ASCII.decode(&encoded, DecoderTrap::Ignore).unwrap();
    println!("{:?}", &decoded);
}

fn read_encoded_txt() {
    println!("READING LINE...");
    let path = Path::new("storage/byte");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut buf = vec![];
    match file.read_to_end(&mut buf) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(s) => s,
    };
    println!("read from file (u8): {:?}", &buf);

    let decoded_str = ASCII.decode(&buf, DecoderTrap::Ignore).unwrap();
    
    println!("decodec string {}", decoded_str);

    // let decoded_str2 = ASCII.decode(&[108, 111, 114, 101, 109, 32, 105, 112, 115, 117, 109], DecoderTrap::Ignore).unwrap();
    
    // println!("decodec string {}", decoded_str2.to_string());
}

fn read_str_write_encoding() {
    println!("READING FILE...");
    let path = Path::new("storage/data");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    let path = Path::new("storage/byte");
    
    let mut file = match File::create(path) {
        Err(why) => panic!("couldn't create file, {}", why.description()),
        Ok(file) => file,
    };

    println!("text: {:?}", &buf);

    // let ve = ASCII.encode(&s, EncoderTrap::Ignore).unwrap();

    // println!("WRITING LINE...");
    // write!(file, "{:?}", ve);
    let mut pos = 0;
    for i in buf {
        let bytes_written = file.write(&[i]);
        pos += bytes_written.unwrap();
    }
}

fn write_buff() -> std::io::Result<()> {
    let mut buffer = BufWriter::new(File::create("storage/byte")?);

    // buffer.write_all(b"some bytes")?;
    // buffer.flush()?;
    let reference = buffer.by_ref();
    // we can use reference just like our original buffer
    reference.write_all(&[108, 111, 114, 101, 109, 32, 105, 112, 115, 117, 109])?;
    Ok(())
}

fn read_buff() -> io::Result<()> {
    let mut f = File::open("storage/byte")?;
    let mut buffer = [0; 10];

    // read up to 10 bytes
    let n = f.read(&mut buffer)?;

    println!("The bytes: {:?}", &buffer[..n]);
    Ok(())
}

fn write_file() {
    println!("WRITING FILE...");
    let path = Path::new("storage/data");
    let s: &str = "lorem ipsum";

    let mut file = match File::create(path) {
        Err(why) => panic!("couldn't create file, {}", why.description()),
        Ok(file) => file,
    };

    file.write_all(s.as_bytes()).expect("couldn't write to file");
}

fn read_byte() {
    let buf = &[0x41u8, 0x41u8, 0x42u8];

    let s = match str::from_utf8(buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("result: {}", s);
}

fn read_file() {
    println!("READING FILE...");
    let path = Path::new("storage/data");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
}

fn sorting_algorithm() {
    let input = user_input();
    let v: Vec<u8> = gen_random(input);

    // print_arr(&v);

    bubble_sort(&v);

    // print_arr(&b_sorted);

    insert_sort(&v);

    // print_arr(&i_sorted);
}

fn bubble_sort(v: &Vec<u8>) -> Vec<u8> {
    let start = Instant::now();
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
    let duration = start.elapsed();
    println!("bubble sort end: {:?}", duration);
    target
}

fn compare_2_val(num1: u8, num2: u8) -> bool {
    if num1 > num2 {
        true
    } else {
        false
    }
}

fn insert_sort(v: &Vec<u8>) -> Vec<u8> {
    let start = Instant::now();
    let mut swap = true;
    let mut target = v.clone();

    while swap {
        swap = false;
        for i in 0..target.len() - 1 {
            if target[i] > target[i + 1] {
                swap = true;
                target.swap(i, i + 1);
            }
        }
    }

    let duration = start.elapsed();
    println!("insertion sort end: {:?}", duration);
    target
}

fn user_input() -> i32 {
    let max_loop = 10000;

    let mut str_input = String::new();
    println!("Enter number to generate for sorting: min = 1, max = 10");
    
    stdin().read_line(&mut str_input).expect("Only numeric number is allowed");

    let mut input:i32 = str_input.trim().parse().unwrap();

    println!("input is {}", input);

    if input > max_loop {
        println!("Number generate to sort not greater than 10");
        input = user_input();
    }

    input
}

fn gen_random(num: i32) -> Vec<u8> {
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

// fn print_arr(v: &Vec<u8>) {
//     for i in v {
//         print!("{}, ", i);
//     }
//     println!("");
// }