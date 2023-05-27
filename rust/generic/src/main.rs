fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// struct Point<T> {
//     x: T,
//     y: T,
// }

// struct Point<T, U> { // 서로 다른 제네릭 유형
//     x: T,
//     y: U,
// }

// enum에서의 제네릭
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
// 유형 T의 값을 유지하는 OK와 유형 E의 값을 유지하는 Err.

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

enum OptionI32 {
    Some(i32),
    None,
}

enum OptionF64 {
    Some(f64),
    None,
}

// use aggregator::{Summary, Tweet};

fn main() {
    let num_list = vec![34, 50, 25, 100, 65];

    // let mut largest = &num_list[0];

    // for number in &num_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }

    let result = largest_i32(&num_list);
    println!("The largest number is {}", result);

    // println!("The largest number is {}", largest);

    let num_list = vec![102, 34, 6000, 69, 54, 2, 43, 8];

    // let mut largest = &num_list[0];

    // for number in &num_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }

    // println!("The largest number is {}", largest);

    let result = largest_i32(&num_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    // 제네릭 타입을 사용한 함수는 i32, char 모두 적용 가능
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
    // 주의 사항
    // 지금은 이 오류가 T가 될 수 있는 모든 가능한 유형에 대해 가장 큰 본문이 작동하지 않을 것이라고 명시한다는 것을 알아두십시오. 
    // 우리는 본문에서 T형의 값을 비교하고 싶기 때문에, 우리는 값이 순서가 될 수 있는 유형만 사용할 수 있습니다. 
    // 비교를 활성화하기 위해 표준 라이브러리에는 유형에 구현할 수 있는 std::cmp:PartialOrd 특성이 있습니다
    // (이 특성에 대한 자세한 내용은 부록 C 참조). 
    // 도움말 텍스트의 제안을 따름으로써, 우리는 T에 유효한 유형을 partialOrd를 구현하는 유형으로만 제한하고, 
    // 표준 라이브러리가 i32와 char 모두에 partialOrd를 구현하기 때문에 이 예제는 컴파일될 것이다.

    // 구조체 정의에서의 제네릭
    // let integer = Point { x: 5, y: 10 };
    // let float = Point { x: 1.0, y: 4.0 };

    // let wont_work = Point { x: 5, y: 4.0 };
    // x 및 y 필드는 둘 다 동일한 일반 데이터 유형 T를 가지고 있으므로 동일한 유형이어야 합니다.

    // let both_integer = Point { x: 5, y: 10 };
    // let both_float = Point { x: 1.0, y: 4.0 };
    // let integer_and_float = Point { x: 5, y: 4.0 }; // 컴파일 완료
    // 이제 표시된 모든 인스턴스가 Point허용됩니다! 
    // 정의에서 제네릭 형식 매개 변수를 원하는 만큼 사용할 수 있지만 몇 개 이상 사용하면 코드를 읽기 어려워집니다. 
    // 코드에 많은 제네릭 형식이 필요한 경우 코드를 더 작은 조각으로 재구성해야 함을 나타낼 수 있습니다.


    // let p = Point { x: 5, y: 10 };

    // println!("p.x = {}", p.x()); // p.x = 5

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // 제네릭을 사용한 코드 성능
    let integer = Some(5);
    let float = Some(5.0);

    let integer = OptionI32::Some(5);
    let float = OptionF64::Some(5.0);
    // 러스트는 제네릭 코드를 각 인스턴스의 유형을 지정하는 코드로 컴파일하기 때문에 제네릭 사용에 대한 런타임 비용을 지불하지 않습니다. 
    // 코드가 실행될 때, 그것은 우리가 손으로 각 정의를 복제한 것처럼 수행된다. 단형화 과정은 러스트의 제네릭을 런타임에 매우 효율적으로 만든다.

    // let tweet = Tweet {
    //     username: String::from("horse_ebooks"),
    //     content: String::from(
    //         "of course, as you probably already know, people",
    //     ),
    //     reply: false,
    //     retweet: false,
    // };

    // println!("1 new tweet: {}", tweet.summarize());
}
