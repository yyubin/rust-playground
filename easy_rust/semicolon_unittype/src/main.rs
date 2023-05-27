// () -> empty tuple, unit type (void)
fn empty_tuple() {
    // 세미콜론이 있으면 empty tuple
    8;
}

fn number() -> i32 {
    8
}

fn main() {
    let my_number = number();
    let tuple = empty_tuple();

    // empty_tuple cant display 
    // display가 안되는 타입인 경우 {:?} or {:#?} 사용, 디버그 프린트
    // println!("{}", tuple);
    println!("{:?}", tuple); // ()

    // main에서 empty_tuple 반환
    tuple;
    
    // main은 리턴 불가능
    //6
}
