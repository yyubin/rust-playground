fn main() {
    // 조건문

    let number = 7;
    
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    //let number = if condition { 5 } else { "six" }; // 변수는 단일 타입을 가져야 한다

    println!("The value of number is: {number}");


    // 반복문
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");


    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // 내부 루프 종료
            }
            if count == 2 {
                break 'counting_up; // 외부 루프 종료
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}"); // 2

    let mut number = 3;

    while number != 0 {
        println!("{number}!!");
        number -= 1;
    }

    println!("LIFTOFF!!!");


    let a = [1, 2, 3, 4, 5];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for element in a { // 컬렉션 순회
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }

    println!("LIFTOFFFF");

}
