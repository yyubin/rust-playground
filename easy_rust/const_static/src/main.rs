// const : 어떤 타입인지 반드시 써야함
const HIGH_SCORE: i32 = 20; // 소문자로 쓸경우 경고 나옴 // global

fn print_high_score() {
    println!("The high score is {}", HIGH_SCORE);
}

// static
// 같은 메모리 공간을 쓴다는 보장, mut 가능 but unsafe
static mut LOWER_SCORE : i32 = 0;

fn main() {
    let x = 8; // 'let' binding: i32
               // &'static str 
               // lifetime에서 static하게 살 수 있는 변수일때 사용하는 개념
    print_high_score();

    unsafe { // 웬만하면 쓰지않기
        LOWER_SCORE = 1;
    }
}
