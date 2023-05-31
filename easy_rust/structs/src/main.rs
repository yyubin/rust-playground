use std::mem::size_of_val;

// struct

// unit struct
struct FileDirectory;

// tuple struct
#[derive(Debug)] // attribute
struct Color(u8, u8, u8);

// name struct
#[derive(Debug)]
struct Country {
    population: u32,
    capital: String,
    leader_name: String
}

#[derive(Debug)]
struct Country2 {
    population: u32,
    capital: String,
    leader_name: String,
    all_populations: [u32; 5500]
}

struct Numbers {
    one: u8,
    two: u8,
    three: u8
}

fn takes_file_directory(input: FileDirectory) {
    println!("file directory");
}

fn main() {
    let x = FileDirectory;
    //takes_file_directory(x);

    println!("size is {}", std::mem::size_of_val(&x)); //size is 0

    let my_color = Color(20, 50, 100);
    println!("color is {:#?}", my_color);
    println!("color is {}", my_color.1);

    let canada = Country {
        population: 35_000_000,
        capital: "Ottawa".to_string(),
        leader_name: "Justin Trudeau".to_string()
    };

    println!("The population is : {}\nThe capital is: {}", canada.population, canada.capital);
    println!("{:#?}", canada);

    let population = 35_000_000;
    let capital = "Ottawa".to_string();
    let leader_name = "Justin Trudeau".to_string();

    let my_country = Country {
        population,
        capital,
        leader_name
    };

    println!("Country is {} bytes in size", size_of_val(&my_country)); // Country is 56 bytes in size

    let number = Numbers {
        one: 2,
        two: 1,
        three: 11
    };

    println!("Size is {}", size_of_val(&number));
    // u8, u8, u8 일 경우 3
    // u8, u8, u8, u32 일 경우 8

    let population = 35_000_000;
    let capital = "Ottawa".to_string();
    let leader_name = "Justin Trudeau".to_string();

    let my_country = Country2 {
        population,
        capital,
        leader_name,
        all_populations: [500; 5500]
    };

    println!("Country is {} bytes in size", size_of_val(&my_country));
    // Country is 22056 bytes in size

}
