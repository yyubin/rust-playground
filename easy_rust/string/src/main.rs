fn main() {
    // growalbe string, owned type
    // String
    
    // &str     ref str     "string slice"
    let my_name = "yubin"; // &str
    let my_name = "yubin".to_string(); // String
    let ohter_name = String::from("yubin2"); // String

    // String = growalbe + shrinkable
    // String = sized type : heap에 있음
    // &str = dynamic type : 스택에 있음
    let mut my_other_name = "yubin3".to_string();
    my_other_name.push('!');
    println!("{my_other_name}"); // yubin3!


    // String의 기능
    // .capacity
    // .push
    // .push_str
    // .pop
    // with_capacity

    let mut my_name = "Yubin".to_string();
    my_name.push('!');
    my_name.push_str(" and I live in Seoul"); // 여러개 push
    println!("{my_name}");

    // allocation / 메모리 할당
    // reallocation / 메모리 재할당 / push를 해서 byte수가 늘어났을 경우 / 보통 2배
    
    let mut my_name = "".to_string();
    println!("Length is {} and capacity is : {}", my_name.len(), my_name.capacity());
    my_name.push_str("yubin");
    println!("Length is {} and capacity is : {}", my_name.len(), my_name.capacity());
    my_name.push('!');
    println!("Length is {} and capacity is : {}", my_name.len(), my_name.capacity());
    my_name.push_str(" and I live in Seoul"); // 여러개 push
    println!("Length is {} and capacity is : {}", my_name.len(), my_name.capacity());
    println!("{my_name}");

    // 메모리 할당
    let mut my_name = String::with_capacity(26);
    println!("Length is {} and capacity is : {}", my_name.len(), my_name.capacity());
    my_name.push_str("yubin");
    println!("Length is {} and capacity is : {}", my_name.len(), my_name.capacity());
    my_name.push('!');
    println!("Length is {} and capacity is : {}", my_name.len(), my_name.capacity());
    my_name.push_str(" and I live in Seoul"); // 여러개 push
    println!("Length is {} and capacity is : {}", my_name.len(), my_name.capacity());
    println!("{my_name}");
    // 26 이상으로 push 하면 2배로 증가 26 -> 52

    // "".to_string() 은 &str을 String으로
    // String::from은 처음부터 String으로
}
