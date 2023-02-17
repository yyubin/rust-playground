use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// 오류 전파
// 함수의 구현이 실패할 수 있을 것을 호출할 때,
// 함수 자체에서 오류 해결하지 않고
// 다른 호출 코드에 오류를 반환하여 수행할 작업을 정할 수 있다
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e), // 파일이 존재하지 않거나 읽을 수 없을 경우 함수는 함수를 호출한 코드에 오류를 반환시킴
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// 오류 전파의 ?연산자
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
// 오류 발생시 ? 연산자는 Err를 일찍 반환하고 호출 코드에 값을 전달함

// 더 짧게 구현
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;
    // 연산자 이후 메서드 호출 연결
    Ok(username)
}

// 더더 짧게 구현
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn main() {
    // 복구 불가능한 오류

    // 패닉 발생시키기
    // 1. 패닉 매크로 호출
    // panic!("crash and burn");
    // 2. 코드의 버그로 호출
    // let v = vec![1, 2, 3];

    // v[99];

    // 복구 가능한 오류
    let greeting_file_result = File::open("hello.txt"); 

    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // 오류시 바로 패닉으로 가기
    let greeting_file = File::open("hello.txt").unwrap();

    // expect는 panic에 전달하는 매개변수
    let greeting_file = File::open("hello.txt")
    .expect("hello.txt should be included in this project");

    // 오류 전파
    // 위쪽으로

    // ?연산자를 사용할 수 있는 경우
    // : 반환 형식이 사용된 값과 호환되는 함수에서만 사용 가능
    let greeting_file = File::open("hello.txt")?; // 오류발생
    // 이 코드는 실패할 수 있는 파일을 엽니다. 
    // ? 연산자는 File::open에서 반환한 Result 값을 따르지만 이 주 함수의 반환 형식은 Result가 아니라 ()입니다.

    // 오류 수정하려면
    // 1. 함수의 반환 유형을 제한이 없는 한 ? 연산자를 사용하는 값과 호환되도록 변경하는 것
    // 2. 일치 또는 결과 <T, E> 방법 중 하나를 사용하여 적절한 방법으로 결과 <T, E>를 처리하는 것
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
    // 이 함수는 옵션 <char>를 반환합니다. 
    // 왜냐하면 문자가 있을 수 있지만 없을 수도 있기 때문입니다. 
    // 이 코드는 문자열 슬라이스 인수를 사용하여 문자열에 있는 줄 위에 반복기를 반환하는 stirng 메소드를 호출합니다. 
    // 이 함수는 첫 번째 줄을 검사하려고 하므로 반복기에서 다음에 호출하여 반복기에서 첫 번째 값을 가져옵니다. 
    // 텍스트가 빈 문자열인 경우 다음 호출은 없음을 반환합니다. 이 경우 ?를 사용하여 last_char_of_first_line에서 없음을 반환합니다. 
    // 텍스트가 빈 문자열이 아닌 경우, 다음은 텍스트의 첫 번째 줄의 문자열 조각을 포함하는 일부 값을 반환합니다.
}

fn main() -> Result<(), Box<dyn Error>> {
    // main은 Result<(), E>도 반환할 수 있습니다. 
    // listing 9-12는 Listing 9-10의 코드를 가지고 있지만, 
    // 우리는 main의 반환 유형을 Result<((), Box<dyn Error>>로 변경하고 반환 값 Ok( )를 끝에 추가했습니다.
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}

// 주 함수가 결과<(), E>를 반환할 때, 주 함수가 Ok()를 반환하면 실행 파일이 0 값으로 종료되고, 
// 주 함수가 Err 값을 반환하면 0이 아닌 값으로 종료됩니다. 
// C로 작성된 실행 파일은 종료 시 정수를 반환한다: 종료된 프로그램은 정수 0을 반환하고, 오류가 발생한 프로그램은 정수 0을 반환한다. 
// 러스트는 또한 이 규칙과 호환되도록 실행 파일에서 정수를 반환합니다.
