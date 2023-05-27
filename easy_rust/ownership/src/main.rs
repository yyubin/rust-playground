// Ownership - 소유권

// fn return_it() -> &'static String {
//     let country = String::from("한국");
//     &country // return &String
// } // country는 여기까지만 살 수 있음 // 데이터가 없어짐

// & immutable reference / shared reference
// &mut mutable reference / unique reference

// & = reference


fn print_country1(country_name: String) {
    println!("My country is {country_name}");
}

fn print_country2(country_name: &String) {
    println!("My country is {country_name}");
}

fn add_is_great(country_name: &mut String) {
    country_name.push_str(" is great!");
    println!("Now it says: {}", country_name);
}

fn add_is_great2(mut country_name: String) { // take by value, declare as mutable
    country_name.push_str(" is great!");
    println!("Now it says: {}", country_name);
}

fn add_is_great3(mut country_name: String) -> String { // take by value, declare as mutable
    country_name.push_str(" is great!");
    println!("Now it says: {}", country_name);
    country_name
}

fn main() {
    let country = String::from("한국");
    let ref_one = &country;
    let ref_two = &country;

    println!("Country is: {}", ref_one);

    //let my_country = return_it(); // error!

    ////// mutable references
    let mut my_number = 9;
    let num_ref = &mut my_number;

    *num_ref = 10;
    println!("Number is now {my_number}");
    
    let mut my_number = 9;
    let num_ref = &mut &mut my_number;

    **num_ref = 10;
    println!("Number is now {my_number}");
    // my_number로 조절 불가능
    // 하나의 데이터를 두곳에서 조작가능하면 unsafe



    ////// references and shadowing

    let mut number = 10;
    let number_ref = &number;
    let number_change = &mut number;
    *number_change += 10;
    //println!("{number_ref}"); // error!!
    println!("{number_change}");

    let mut number = 10;
    let number_change = &mut number;
    *number_change += 10;
    let number_ref = &number;
    println!("{number_ref}"); 


    //// shadowing
    let country = "대한민국";
    let country_ref = &country;
    let country = 8;
    println!("{}, {}", country_ref, country); // 대한민국, 8
    // country_ref를 이용해서 쉐도잉되기전 데이터 확인 가능



    // references in function
    let country = "대한민국".to_string();
    print_country1(country); // 소유권이 이 함수에게로 옮겨짐
                            // 다시 프린트를 country로 하는 것이 불가능
                            // 리턴 받아서 가능하긴한데 이상함
    //print_country(country); // 두번 호출시 에러

    let mut country = "대한민국".to_string();
    print_country2(&country);
    print_country2(&country);
    print_country2(&country);
    // ref로 전달하면 여전히 country가 소유권을 가진다..


    //////
    /// mutable references and mut in functions
    //////
    let mut my_country = "Canada".to_string();
    add_is_great(&mut my_country); // by mutable reference
    add_is_great(&mut my_country);


    let my_country = "한국".to_string();
    add_is_great2(my_country);

    let my_country = "한국".to_string();
    let tmp = add_is_great3(my_country);
    println!("{}", tmp); // 한국 is great!
    // 값으로 빌려주고 mut로 받아서 push 한 후 다시 stirng으로 소유권 받아서 보여줌

}
