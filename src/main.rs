use std::io;  // prelude
use rand::Rng; // trait
use std::cmp::Ordering;

fn chapter3() {
    let mut x = 1;
    x = 10;
    println!("{}", x);

    // shadow
    let s = "aa";
    let s = s.len();
    println!("{}", s);

    // 类型推断
    // let x: = "12".parse().expect("不是数字");
    let x: u32 = "12".parse().expect("不是数字");
    println!("{}", x);
}

fn main() {
    chapter3();
    // chapter2();
}

fn chapter2() {
    println!("猜测一个数，这个数是?");
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("请输入：");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("读取输入错误");
        // shadow
        // let guess: u32 = guess.trim().parse().expect("请输入数字");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("you win!!");
                break;
            },
            Ordering::Less => println!("too small!!"),   // arm
            Ordering::Greater => println!("too big!!"),
        }    
    }
}