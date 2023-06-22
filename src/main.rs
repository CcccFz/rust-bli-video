use std::io;  // prelude
use rand::Rng; // trait
use std::cmp::Ordering;

fn chapter6() {
    // enum
    enum Msg {
        Quit,
        Move {x:i32, y:i32},
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    impl Msg {
        fn call(&self) {

        }
    }

    let q = Msg::Quit;
    let m = Msg::Move { x: 1, y: 1 };
    let w = Msg::Write(String::from("Hello"));
    let c = Msg::ChangeColor(0,255,255);
    m.call();

    // Option
    let same_number = Some(5);
    let same_str = Some("A String");
    let absent_number: Option<i32> = None;

    // match option
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i+1),
            None => None,
        }
    }
    let six = plus_one(Some(5));
    let none = plus_one(None);

    let v = 0u8;
    match v {
        1 => println!("one"),     
        _ => (), 
    }

    let v = Some(0u8);
    if let Some(3) = v {
        println!("three");
    } else {
        println!("others");
    }
}

fn main() {
    chapter6();
    // chapter5();
    // chapter4();
    // chapter3();
    // chapter2();
}

fn chapter5() {
    // 结构体方法
    struct Rectangle {
        width: i32,
        height: i32,
    }
    impl Rectangle {
        fn area(&self) -> i32 {
            self.width * self.height
        }
        fn square(size: i32) -> Rectangle {
            Rectangle { width: size, height: size }
        }
    }
    let ret1 = Rectangle{
        width: 20,
        height: 30,
    };
    let ret2 = Rectangle::square(20);
    println!("{}", ret1.area());
    println!("{}", ret2.area());
}

fn chapter4() {
    // String
    println!("String:");
    let mut s = String::from("hello");
    s.push_str(" world");
    println!("{}", s);

    // move
    println!("move:");
    let s1 = String::from("aa");
    let s2 = s1;
    let s3 = s2.clone();
    // println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);

    // 标量无move
    println!("标量无move:");
    let n1 = 100;
    let n2 = n1;
    println!("{}", n1);
    println!("{}", n2);

    // move and func
    println!("move and func:");
    fn handle_str(s1: String) {
        println!("{}", s1);
    }
    fn handle_num(n1: i32) {
        println!("{}", n1)
    }
    let s1 = String::from("aa");
    let n1 = 5;
    handle_str(s1);
    handle_num(n1);
    // println!("{}", s1);
    println!("{}", n1);

    // reference and borrow
    println!("reference and borrow:");
    fn calc_length(s: &String) -> usize {
        // s.push_str("bb");
        s.len()
    }
    let s1 = String::from("aa");
    let len = calc_length(&s1);
    println!("{} {}", s1, len);

    // mut reference and borrow
    println!("mut reference and borrow:");
    let s = String::from("bb");
    let s1 = &s;
    let s2 = &s;
    println!("{}", s1);
    println!("{}", s2);
    let mut s  = String::from("bb");
    let s1 = &mut s;
    // let s2 = &mut s;
    println!("{}", s1);
    // println!("{}", s2);

    // 可通过创建新的作用域，来允许非同时创建多个可变引用
    println!("可通过创建新的作用域，来允许非同时创建多个可变引用");
    let mut s = String::from("cc");
    { let s1 = &mut s; }
    let s2 = &mut s;

    // 不可以同时拥有一个可变引用和一个不变引用
    println!("不可以同时拥有一个可变引用和一个不变引用");
    let mut s = String::from("zz");
    let s1 = &s;
    let s2 = &s;
    // let s3 = &mut s;
    println!("{}", s1);
    println!("{}", s2);

    // 悬空引用
    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //     &s
    // }
    // let r = dangle();

    // 更喜欢用字符串切片作为，函数参数
    fn first_world(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
        }
        return &s[..];
    }
    let my_string = String::from("hello world");
    let word = first_world(&s[..]);
    let my_string = "hello world";
    let word = first_world(my_string);
}

fn chapter3() {
    println!("mut:");
    let mut x = 1;
    x = 10;
    println!("{}", x);

    // shadow
    println!("shadow:");
    let s = "aa";
    let s = s.len();
    println!("{}", s);

    // 类型推断
    println!("类型推断:");
    // let x: = "12".parse().expect("不是数字");
    let x: u32 = "12".parse().expect("不是数字");
    println!("{}", x);

    // tup
    println!("tup:");
    let tup: (i32, u64, f64) = (20, 50, 6.4);
    let (x, _, z) = tup;
    println!("{} {} {}", x, tup.1, z);

    // 表达式
    println!("表达式:");
    // let y = (let x = 1);
    let y = {
        let x = 1;
        // x + 3;
        x + 3
    };
    println!("{}", y);

    fn five() -> u32 {
        // 5;
        5
    }
    let x = five();
    println!("{}", x);

    // for
    println!("for:");
    let arr = [1, 2, 3];
    for x in arr.iter() {
        println!("{}", x);
    }
    for x in (1..4).rev() {
        println!("{}", x);
    }
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