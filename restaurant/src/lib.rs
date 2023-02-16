mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// use crate::front_of_house::hosting; // use 키워드 사용시 '특정 범위에 대한 바로가기' 생성
use crate::front_of_house::hosting::add_to_waitlist;

mod customer {
    pub fn eat_at_restaurant() {
        //hosting::add_to_waitlist(); // 이대로 컴파일시 오류발생, use가 해당범위에서 더이상 사용되지 않음
                                    // 해결: super::hostingcustomer / 하위 모듈에서 customer 사용하기
        add_to_waitlist();
    }
}


fn deliver_order() {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::deliver_order(); // super 사용시 상대경로로 시작
//     }

//     fn cook_order() {}
// }

// 구조체와 열거형 공개
mod back_of_house {
    pub enum Appetizer { // 열거형은 디폴트가 pub
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

// 일부 공개 필드와 일부 비공개 필드가 있는 구조체
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    super::hosting customer
}