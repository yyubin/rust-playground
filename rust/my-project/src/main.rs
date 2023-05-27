use crate::garden::vegetables::Asparagus;
use std::collections::HashMap;

pub mod garden

// use std::fmt; // 이름 같은 모듈 사용시 부모 모듈 이름으로 use
// use std::io;

use std::fmt::Result; // as 키워드로 새이름 제공 가능
use std::io::Result as IoResult; 

// // --snip--
// use std::cmp::Ordering;
// use std::io;
// // --snip--

// 위 모듈 use 하위 처럼 중첩 가능
// --snip--
use std::{cmp::Ordering, io};
// --snip--

// 글롭 연산자 가능
use std::collections::*;

// fn function1() -> fmt::Result {
//     // --snip--
// }

// fn function2() -> io::Result<()> {
//     // --snip--
// }

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);

    // 관용적 use 경로
    let mut map = HashMap::new();
    map.insert(1, 2);
}
