// #[derive(Debug)]
// enum IpAddrKind{
//     V4(u8, u8, u8, u8),
//     V6(String)
// }
// enum IpAddrKind{
//     V4(String),
//     V6(String),
// }
// enum IpAddrKind {
//     V4,
//     V6
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// fn route(ip_kind: IpAddrKind) {
//     println!("IP kind is : {:?}", ip_kind);
// }

// 표준라이브러리의 IP주소 인코딩의 정의
struct Ipv4Addr {

}

struct Ipv6Addr {

}

enum IpAddr{
    V4(Ipv4Addr), // enum에 구조체 넣을 수 있음 
    V6(Ipv6Addr),
}

// 다른 enum 예시
enum Message {
    Quit,   // 데이터 없음
    Move { x: i32, y: i32 }, // 구조체 처럼 명명된 필드
    Write(String), // 단일 string
    ChangeColor(i32, i32, i32), // i32의 3가지 값 포함
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

impl Message { // message 메소드
    fn call(&self) {
        println!("Message called");
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // route(IpAddrKind::V4); // IP kind is : V4
    // route(IpAddrKind::V6); // IP kind is : V6

    // let home = IpAddr {
    //     kind: four,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: six,
    //     address: String::from("::1"),
    // };

    // let home = IpAddr::V4(String::from("127.0.0.1")); // enum 구조에 String 인자 추가하여 구성하는게 더 간결하다
    // let loopback = IpAddr::V6(String::from("::1"));

    // let home = IpAddr::V4(127,0,0,1);
    // let loopback = IpAddr::V6(String::from("::1")); // 각 변수마다 다른 데이터 유형으로 가능, 구조체에서는 불가능하다

    let m = Message::Write(String::from("Hello"));
    m.call();

    // 열거형의 Option과 Null 값에 대한 이점
    // Rust에는 다른 많은 언어에 있는 null 기능이 없습니다.
    // null이 있는 언어에서는 변수는 항상 null이거나 null이 아닌 상태일 수 있다.
    // null인 값을 null이 아닌 값으로 사용하려고 할 때 오류 발생

    // Rust에서는 null이 없지만 존재하거나 존재하지 않는 값의 개념을 인코딩 할 수 있는 열거형이 있다.
    // 표준 라이브러리의 정의
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }

    let some_number = Some(5);  // Option<i32>
    let some_char = Some('e');  // Option<char> rust에서 유형을 추론할 수 있다

    let absent_number: Option<i32> = None;
    // rust에서 Option<i32>을 추가하여야 하는데 그 이유는 None으로는 유형을 추론할 수 없기 때문

    // Option<T>와 T(T는 모든 유형이 가능한 제네릭)는 다른 유형이기 때문에
    // Option<T>가 확실히 유효한 값인 것처럼 사용하도록 허용하지 않는다

    let x: i8 = 5;
    // let y: Option<i8> = Some(5); // 컴파일 불가 코드
    // // Option<i8>는 값이 없을 가능성을 걱정해야함 
    // // 작업을 수정하려면 Option<T>로 바꾸어야 한다.
    // // 이는 실제로 null이 아니지만 null이 아니라고 가정하는 것이다
    // // 값을 사용하려면 Option<T> 각 변형을 처리할 코드가 필요함!

    // let sum = x + y;
    // println!("sum is {}", sum);

    let coin = Coin::Penny;
    value_in_cents(coin); // Lucky Penny!

    value_in_cents(Coin::Quarter(UsState::Alaska)); //State quarter from Alaska!

    
    let five = Some(5);
    let six = plus_one(five); // return 6
    let none = plus_one(None); // return None


    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // 3과 7이 아닐 경우 아무 것도 실행하지 않음
    }

    fn add_fancy_hat() {
        println!("add fancy hat");
    }
    fn remove_fancy_hat() {
        println!("remove fancy hat");
    }

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    // result : The maximum is configured to be 3

    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    // result : The maximum is configured to be 3
    // if let을 활용하면 _ => () 생략가능

    let other_coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;
    match other_coin {
        Coin::Quarter(ref state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    // result : State quarter from Alaska!

    if let Coin::Quarter(ref state) = other_coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    // result : State quarter from Alaska!

    println!("{}", count); // 0
}
