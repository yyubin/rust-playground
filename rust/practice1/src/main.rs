use std::io;

fn main() {
    // 화씨와 섭씨 온도 변환
    let c = 36.0;
    let f = ctof(c);
    println!("섭씨 {c}도는 화씨 {f}도 입니다");

    let another_f = 187.0;
    let another_c = ftoc(another_f);
    println!("화씨 {another_f}도는 섭씨 {c}도 입니다");

    // n번째 피보나치 수
    let mut n = String::new();
    
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: i32 = n
        .trim()
        .parse()
        .expect("n entered was not a number");


    match n {
        1 => println!("1"),
        2 => println!("1"),
        _ => fibo(n),
    }

    // 크리스마스 캐롤
    // On the first day of Christmas my true love sent to me
    // A partridge in a pear tree

    // On the second day of Christmas my true love sent to me
    // Two turtle doves,
    // And a partridge in a pear tree.

    // On the third day of Christmas my true love sent to me
    // Three French hens,
    // Two turtle doves,
    // And a partridge in a pear tree.
    let lyrics = ["A partridge in a pear tree", "Two turtle doves,", "Three French hens,"];
    let arr = ["first", "second", "third"];
    let mut index = 0;

    for _ in (0..3) {
        println!("On the {} day of Christmas my true love sent to me", arr[index]);
        for i in (0..(index + 1)).rev() {
            println!("{}", lyrics[i]);
        }
        index += 1;
        println!();
    }


}

fn fibo(n: i32) {
    let mut i = 1;
    let mut j = 1;
    let mut tmp = 0;
    for _ in 2..n {
        tmp = i + j;
        i = j;
        j = tmp;
    }
    println!("{}", tmp);
}

fn ctof(c: f32) -> f32 {
    c * 1.8 + 32.0
}

fn ftoc(f: f32) -> f32 {
    (f - 32.0) * 5.0/9.0
}
