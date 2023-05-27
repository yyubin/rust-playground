fn main() {
    let width1 = 30;
    let height1 = 50;
    // println!("The area of the rectangle is {} square pixels.", area(width1, height1));

    //Refactoring with Tuples
    let rect1 = (30, 50);
    // println!("The area of the rectangle is {} square pixels.", area(rect1));
    // 튜플 사용시 구조 추가, 하나의 인수만 전달 가능
    // 튜플은 요소의 이름을 지정하지 않으므로 튜플 일부를 인덱싱하기 때문에 계산이 덜 명확함

    //Refactoring with Structs: Adding More Meaning
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
        // 소유권을 얻기보다 빌리고 싶음
        // 이와 같이하면 main의 소유권을 잃지 않을 수 있다
    );

    // Adding Useful Functionality with Derived Traits
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2);

    // Method
    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect3.area()
    );

    if rect3.width() {
        println!("The rectangle has a nonzero width; it is {}", rect3.width);
    };

    let rect4 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect5 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect6 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect4 hold rect5? {}", rect4.can_hold(&rect5));
    println!("Can rect4 hold rect6? {}", rect4.can_hold(&rect6));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

fn area(rectangle: &Rectangle) -> u32 { 
    rectangle.width * rectangle.height
}

impl Rectangle {
    fn area(&self) -> u32 {
        // &Rectangle 대신 &self를 사용합니다. &self는 사실 self: &self의 줄임말이다.
        // 메서드의 첫 번째 매개 변수에는 Self 유형의 self라는 매개 변수가 있어야 하므로 
        // Rust를 사용하면 첫 번째 매개 변수 위치에 self라는 이름만 사용하여 이를 약어화할 수 있습니다.
        // &Rectangle에서처럼 이 메서드가 자체 인스턴스를 차용한다는 것을 나타내려면 자체 속기 앞에 &를 사용해야 합니다.
        // 메소드가 수행하는 작업의 일부로 메소드를 호출한 인스턴스를 변경하려면 첫 번째 매개 변수로 &mut self를 사용합니다.
        self.width * self.height
    }
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        // if self.width > other.width && self.height > other.height {
        //     return true;
        // }
        // return false;
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Self {
        // self로 작업할게 없는 경우 첫 번째 매개변수가 없는(따라서 메소드가 아닌) 연관된 함수 정의 가능
        // 정사각형 만들 경우 인수를 2번 입력하지 않고 생성가능
        Self {
            width: size,
            height: size,
        }
    }
}

//구조체를 사용하면 도메인에 의미 있는 사용자 지정 유형을 만들 수 있습니다. 
//구조체를 사용하면 연결된 데이터 조각을 서로 연결하고 각 조각의 이름을 지정하여 코드를 명확하게 만들 수 있습니다. 
//블록 에서 impl유형과 연결된 함수를 정의할 수 있으며 메서드는 구조체 인스턴스의 동작을 지정할 수 있는 일종의 연결된 함수입니다.