// Vec
// Vec<String>
// T = some types / generics

fn main() {
    let mut my_vec: Vec<String> = Vec::new();
    let name1 = String::from("Windy");
    let name2 = String::from("Gomesy");
    let name3 = String::from("Wendy");

    let mut my_vec = Vec::new();
    println!("Space for my_vec: {}", my_vec.capacity());
    my_vec.push(name1.clone());
    println!("Space for my_vec: {}", my_vec.capacity());
    my_vec.push(name2.clone());
    println!("Space for my_vec: {}", my_vec.capacity());
    my_vec.push(name3);
    println!("Space for my_vec: {}", my_vec.capacity());

    println!("My cats are {:?}", my_vec);


    let my_vec2 = vec![name1, name2];
    println!("My cats are {:?}", my_vec2);

}
