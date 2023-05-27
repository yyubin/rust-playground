enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // 벡터 
    // 벡터 사용시 모든 값을 메모리에 나란히 배치 할 수 있음
    let v: Vec<i32> = Vec::new(); // i32와 같은 타입 필요
    let v = vec![1, 2, 3]; // 타입 불필요

    let mut v = Vec::new(); // mut로 선언시 들어가는 값으로 유형 추론 가능

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }


    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    // v.push(6);

    println!("The first element is: {first}"); // 오류 발생
    // 벡터는 메모리 옆에 새로운 값을 넣기 때문에 벡터 끝에 새로 할당하면
    // 새로운 메모리 공간 할당 후 복사
    // println에서의 first는 이미 해제된 메모리 가리킴

    let v = vec![100, 32, 57];
    for i in &v{
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v{
        *i += 50;
    }

    // 열거형 사용하여 여러 유형 저장
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
    // 벡터도 범위 벗어나면 메모리 해제

    // 문자열
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    let s = String::from("initial contents");
    // to_string 과 from은 스타일과 가독성 정도의 차이

    // 문자열 업데이트
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}"); // s2 is bar

    let mut s = String::from("lo");
    s.push('l');

    // 문자열 연산자
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; 
    // 이 경우 s1 더이상 유효하지 않음
    // s2는 유효함 
    // 추가할 호출에서 &s2를 사용할 수 있는 이유는 컴파일러가 &String 인수를 강제로 a &str로 만들 수 있기 때문입니다. 
    // 우리가 add 메소드를 호출할 때, 러스트는 deref 강제력을 사용하는데, 여기서 &s2를 &s2[...]로 변환한다.
    // add는 s 매개 변수의 소유권을 갖지 않으므로 이 작업 후에도 s2는 여전히 유효한 문자열이 됩니다.

    // s3 = s1 + &s2;가 두 문자열을 모두 복사하고 새 문자열을 만들 것처럼 보이지만, 
    // 이 문은 실제로 s1의 소유권을 가져와서 s2의 내용 복사본을 추가한 다음 결과의 소유권을 반환합니다.

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}"); // 복잡한 문자열 결합에서는 포맷사용 가능, 소유권 가지지 않음

    // 문자열로 인덱싱
    let s1 = String::from("hello");
    // let h = s1[0]; // rust는 문자열 인덱싱 불가능, 오류 발생

    let hello = String::from("Hola");
    // 문자열 저장한 벡터의 길이 : 4바이트
    let hello = String::from("Здравствуйте");
    // 문자열 저장한 벡터의 길이 : 24바이트
    // 해당 문자열의 각 유니코드 스칼라 값은 2바이트의 저장 공간을 차지하기 때문

    // Rust에서 문자열을 보는 3가지 방법
    // 바이트, 스칼라값, 문자소 클러스터
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    // char에 해당 하는 바이트 : ['न', 'म', 'स', '्', 'त', 'े']
    // 문자소 클러스터 : ["न", "म", "स्", "ते"]

    // 문자열 슬라이싱
    let hello = "Здравствуйте"; // 문자마다 각 2바이트

    let s = &hello[0..4]; // Зд

    // 문자열 반복
    for c in "Зд".chars() {
        println!("{c}");
    }


    // 해시 맵
    // 키와 값 을 메모리에 배치하는 방법을 결정하는 해싱 함수를 사용하여 유형 HashMap<K, V> 의 키와 유형의 값의 매핑을 저장
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0); // 10

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // 해시 맵 소유권
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 값이 이동되고 해시 맵이 해당 값의 소유자가 됨

    // 해시 맵 업데이트
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // 값 덮어쓰기

    println!("{:?}", scores);

    // 키 없는 경우에만 값 추가
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // 이전 값을 기반으로 업데이트
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    } // 단어 발생 카운트

    println!("{:?}", map); // {"wonderful": 1, "world": 2, "hello": 1}
}
