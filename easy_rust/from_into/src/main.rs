// trait
// This type implements (trait name)
// 

// From, Into

fn main() {
    let my_name = String::from("yubin");
    let my_city: String = "Seoul".into();

    println!("{}", my_city);
    my_city.aaaa(); //no method named `aaaa` found for struct `String` in the current scope
    // String!

    let my_vec = Vec::from([8, 9, 10]); //[i32, 3]

}
