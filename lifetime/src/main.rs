fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// 이 함수를 정의할 때 이 함수에 전달될 구체적인 값을 모르기 때문에 if 케이스가 실행될지 다른 케이스가 실행될지 알 수 없습니다. 
// 또한 전달될 참조의 구체적인 수명도 모르기 때문에, 목록 10-17 및 10-18에서와 같이 범위를 검토하여 반환되는 참조가 항상 유효한지 여부를 결정할 수 없습니다. 
// 대여 검사기는 x와 y의 수명이 반환 값의 수명과 어떻게 관련되어 있는지 모르기 때문에 이것도 확인할 수 없습니다. 
// 이 오류를 해결하기 위해 차용 검사기가 분석을 수행할 수 있도록 참조 간의 관계를 정의하는 일반 수명 매개 변수를 추가합니다.

// Lifetime Annotation Syntax
// 수명 주석은 참조의 수명을 변경하지 않습니다. 
// 오히려, 그들은 수명에 영향을 미치지 않고 서로에 대한 여러 참조의 수명 관계를 기술한다. 
// 서명이 일반 유형 매개 변수를 지정할 때 함수가 모든 유형을 허용할 수 있는 것처럼 함수는 일반 수명 매개 변수를 지정하여 모든 수명의 참조를 허용할 수 있습니다.

// 수명 매개변수의 이름은 아포스트로피(')로 시작해야 하며 일반적인 유형처럼 모두 소문자이고 매우 짧습니다. 
// 대부분의 사람들은 생애 첫 주석에 'a'라는 이름을 사용한다. 
// 우리는 참조의 & 뒤에 수명 매개 변수 주석을 배치하며, 참조 유형에서 주석을 분리하기 위한 공간을 사용한다.

// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

// 가장 긴 시간에 대한 구체적인 참조를 전달할 때 'a'로 대체되는 구체적인 수명은 x의 범위 중 y의 범위와 겹치는 부분입니다. 
// 즉, 일반적인 수명 'a'는 x와 y의 수명 중 더 작은 것과 같은 구체적인 수명을 얻을 것이다. 
// 동일한 수명 매개 변수 'a'로 반환된 참조에 주석을 달았기 때문에 반환된 참조는 x와 y의 수명 중 더 작은 길이에도 유효하다.

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    // The Borrow Checker
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
                          // ----------+

    // Generic Lifetimes in Functions
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result); //The longest string is abcd

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result); // The longest string is long string is long
    }

    // 하위 코드는 오류 발생
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str()); 
    // }
    // println!("The longest string is {}", result);
    // 우리는 이 코드를 보고 string1이 string2보다 길다는 것을 알 수 있으며 따라서 result는 string1에 대한 참조를 포함할 것이다. 
    // string1이 아직 범위를 벗어나지 않았기 때문에 string1에 대한 참조는 println! 문에 여전히 유효합니다. 
    // 그러나 컴파일러는 이 경우 참조가 유효한지 확인할 수 없습니다. 
    // 우리는 Rust에게 가장 긴 함수에 의해 반환된 참조의 수명이 전달된 참조의 수명 중 더 작은 것과 같다고 말했다. 
    // --> 수명이 더 작은 쪽의 변수를 따라가게 된다 
    // 따라서 차용 검사기는 목록 10-23의 코드가 잘못된 참조를 갖는 것으로 허용하지 않습니다.
    // https://velog.io/@0xjuchan/Rust-Lifetime

    // Thinking in Terms of Lifetimes
    
    // Lifetime Annotations in Struct Definitions
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // Lifetime Elision
    // 지금까지 함수에서 수명주기 인수가 필요하지 않았던 이유
    // Rust의 참조 분석에 프로그래밍된 패턴을 수명 제거 규칙 이라고 합니다 . 이것은 프로그래머가 따라야 할 규칙이 아닙니다. 
    // 이들은 컴파일러가 고려할 특정 사례의 집합이며, 코드가 이러한 경우에 적합하면 수명을 명시적으로 작성할 필요가 없습니다.
    // 제거 규칙은 완전한 추론을 제공하지 않습니다. Rust가 결정적으로 규칙을 적용하지만 참조의 수명에 대해 여전히 모호한 경우 컴파일러는 나머지 참조의 수명을 추측하지 않습니다. 
    // 추측하는 대신 컴파일러는 수명 주석을 추가하여 해결할 수 있는 오류를 제공합니다.
    // 함수 또는 메소드 매개변수에 대한 수명을 입력 수명 이라고 하고 반환 값에 대한 수명을 출력 수명 이라고 합니다 .
    // https://blog.seulgi.kim/2019/12/rust-lifetime-elision.html << 읽어보기!!

    // The Static Lifetime
    // 이 문자열의 텍스트는 항상 사용 가능한 프로그램의 이진 파일에 직접 저장됩니다. 따라서 모든 문자열 리터럴의 수명은 '정적'이다.
    // 오류 메시지에 '정적 수명'을 사용하라는 제안이 표시될 수 있습니다. 
    // 그러나 'static'을 참조 수명으로 지정하기 전에 프로그램의 전체 수명을 실제로 사용하는지 여부와 원하는 경우를 생각해 보십시오. 
    // 대부분의 경우 '정적 수명'을 나타내는 오류 메시지는 사용 가능한 수명의 불일치 또는 매달린 참조를 생성하려고 시도한 결과 발생합니다. 
    // 이러한 경우 해결책은 '정적 수명'을 지정하는 것이 아니라 이러한 문제를 해결하는 것입니다.

    

}

