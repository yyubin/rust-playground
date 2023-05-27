struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,       //username: username, // 필드 초기화 약어 사용 가능
        email,          //email: email,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32)
struct Point(i32, i32, i32)

struct AlwaysEqual;

fn main() {
    let mut user1 = User { // 특정 필드만 가변으로 지원하지 않음
        active: true,
        username: String::from("someone123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    // '..' 구문은 명시적으로 설정되지 않은 나머지 필드를 지정된 인스턴스의 필드와 동일한 값 갖도록 함
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    
    // 구조체 업데이트 구문은 '=' 할당처럼 사용, 데이터를 이동시킴, user1 사용 불가
    // email 이 다르기 때문에 하단 코드도 현재는 작동
    //     let user2 = User {
    //     ..user1
    // }; -> 와 같이 한다면 하단 코드 에러 발생함
    println!("{}", user1.email);
    

    // 명명된 필드 없이 튜플 구조체를 사용하여 다른 유형 만들기
    let black = Color(0, 0, 0);
    let origin = Color(0, 0, 0);
    // black과 origin 은 서로 다른 튜플 구조체의 인스턴스이기 때문에 서로 다른 유형

    // 필드가 없는 단위 유사 구조체
    let subject = AlwaysEqual;
}
