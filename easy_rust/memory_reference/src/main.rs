// memory
// stack 메모리
// 위로 쌓이는 메모리
// 함수 리턴이 나오면 메모리 회수
// 빠름
// 간단한 타입, i32, char, bool, etc..


// heap 메모리
// 스택에 넣을 수 없을 경우 힙에 넣음
// pointer를 이용
// 공간이 많음
// rust에서는 reference라는 포인터 사용
// reference는 책의 목차와 같다
// reference는 빌리는 것

fn main() {
    let my_number = 15; // This is an i32

    // reference to my_number
    let single_reference = &my_number; //  This is a &i32

    let double_reference = &single_reference; // This is a &&i32
    let five_references = &&&&&my_number; // This is a &&&&&i32

    println!("{single_reference}"); // 15
    println!("{double_reference}"); // 15
    println!("{five_references}"); // 15
}