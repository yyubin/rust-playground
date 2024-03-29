//https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

fn main() {
    {
                            // s is not valid here, it’s not yet declared
        let s = "hello";    // s is valid from this point forward

        // do stuff with s
    }                       // this scope is now over, and s is no longer valid

    let s = String::from("hello");

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // hello, wolrd!

    //  With the String type, in order to support a mutable, growable piece of text, 
    // we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. -> string을 힙 메모리에 저장해야 한다
    // 런타임시 메모리 할당자에 메모리 할당을 요청해야 한다

    // String::from -> 구현이 필요한 메모리 할당을 요청, 다른 언어에서도 보편적
    // GC도 사용하지 않고, 명시적으로 할당 및 해제를 하지도 않는 대신 메모리를 소유한 변수가 범위를 벗어나면 메모리가 자동으로 반환된다

    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }                                  // this scope is now over, and s is no
                                       // longer valid
    // 변수가 범위를 벗어나면 rust는 'drop'함수 호춡 괄호가 닫히는 부분에서 자동으로 호출하여 메모리 반환함


    let x = 5;
    let y = x; // x의 복사본을 만들어 y에 바인딩
    // 위 두값은 스택에 푸쉬됨

    let s1 = String::from("hello"); 
    let s2 = s1; // s1의 복사본을 만들어 s2에 바인딩 하는 '것처럼' 보임
    // s1에는 문자열의 내용을 가지는 메모리에 대한 포인터, 길이, 용량에 대한 정보가 있음
    // s2에도 그 값이 복사되기 때문에 둘은 같은 메모리 주소를 가리키고 있다.
    // s1은 스택메모리, 실제 데이터는 힙메모리에 위치함

    // 힙메모리의 내용까지도 복사하게 되면 깊은복사, 포인터와 주솟값만 가지는 스택메모리만 복사하면 얕은복사
    // 깊은 복사 사용시 작업 비용이 커진다

    // 만약 s1, s2가 범위를 벗어나면 둘 다 동일한 메모리를 해제하려고 할 것
    // 이것은 이중 자유 오류 -> 잠재적 보안 취약
    
    // 메모리 안전을 보장하기 위해 rust에서는 let s2 = s1; 에서 s1은 더이상 유효하지 않는 것으로 간주한다
    // rust는 얕은 복사를 사용하는게 아님, s1까지 무효화 하기 때문, 이동이라고 함
    // s1이 s2로 이동됨
    // 유효한 경우에만 s2가 범위를 벗어나면 단독으로 메모리가 해제되고 작업이 완료된다
    // 깊은 복사도 사용하지 않기 때문에 런타임 성능 측면에서 유리함

    
    // == 클론과 상호 작용하는 변수 및 데이터 ==
    // 깊은 복사 : 스택 데이터 뿐만아니라 힙 데이터까지 복사하는 방법

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // 스택 데이터 복사
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y); // x = 5, y = 5
    // 위 상황에서 x는 유효함, '이동'되었지만 x가 출력 된다
    // 컴파일 타임에 알려진 크기를 갖는 정수와 같은 유형이 스택에 완전히 저장되기 때문에 실제 값의 복사본을 빠르게 만들 수 있기 때문


    let s = String::from("hello");
    
    takes_ownership(s);
    // s는 더이상 유효하지 않음
    // s를 호출하거나 사용하려하는 경우 컴파일 오류 발생

    let x = 5;

    makes_copy(x);
    // x는 이 이후에도 계속 사용 가능

    let s1 = gives_ownership(); // 함수 호출시 반환을 받으면 다시 사용 가능.

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);



    // let s1 = String::from("hello");

    // let (s2, len) = calculate_length(s1); // tuple을 반환받아 사용할 수도 있음

    // println!("The length of '{}' is {}.", s2, len);

    // 함수가 값을 사용하지만 소유권은 가지지 않도록 구성할 수도 있다.. next..

    let s1 = String::from("hello");

    let len = calculate_length(&s1);
    // &s1 사용시 값을 참조하지만 소유하지 않는 참조를 만들 수 있다
    // s1을 소유하지 않기 때문에 참조가 사용을 중단해도 s1이 사라지지 않는다

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
    // 참조 후 변경하고자 하면 &mut 구문사용


    let mut s = String::from("Hello");

    // let r1 = &mut s;
    // let r2 = &mut s;
    // 두개의 변경가능한 참조는 만들 수 없다
    {
        let r1 = &mut s;
    }
    let r2 = &mut s;
    // scope를 다르게 하여 해결 가능

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // problem
    // 동일한 값 에 대한 불변 참조가 있는 동안에는 가변 참조를 가질 수 없습니다.
    // 하지만 사용되지 않아 메모리가 반환되면 다시 사용 가능

    let reference_to_nothing = dangle();

    // let mut s = String::from("hello world");

    // let word = first_word(&s); // 5

    // s.clear(); // s = ""
    // frist_word 함수 바뀌면서 s.clear()는 불가능해짐
    // 무언가에 대한 불변 참조가 있으면 가변 참조도 사용할 수 없음

    // slice
    let s = String::from("Hello World");

    let hello = &s[0..5];
    let world = &s[6..11];
    
    let slice = &s[0..2];
    let slice = &s[..2]; // 0 생략 가능

    let s = String::from("hello");

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..]; // 마지막 생략 가능

    let slice = &s[0..len];
    let slice = &s[..]; // 동일 구문

    let my_string = String::from("hello world");

    let word = first_word(&my_string[0..6]); 
    let word = first_word(&my_string[..]);
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    let word = first_word(my_string_literal); // 모두 실행 가능
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // 이 지점에서 drop 호출, 
// memroy is freed.

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // integer의 경우에는 아무일도 일어나지 않음


fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
        // moves out to the calling
        // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
             // scope

    a_string  // a_string is returned and moves out to the calling function
}

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() returns the length of a String

//     (s, length)
// }

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

fn dangle() -> String {
    let s = String::from("hello");

    s
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes(); // 바이트로 변환

//     for (i, &item) in bytes.iter().enumerate() { // 바이트 배열에 대한 반복자
//         if item == b' ' { // 공백 위치 찾으면 반환
//             return i;
//         }
//     }

//     s.len()
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}