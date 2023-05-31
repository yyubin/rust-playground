// tuples
// Vec<String>

fn main() {
    let my_tuple = (8, "yubin", vec![8, 9]);
    // my_tuple.aaaa(); // type : ({integer}, &str, Vec<{integer}>)
    // no method named `aaaa` found for tuple `({integer}, &str, Vec<{integer}>)` in the current scope

    let x = (); // empty tuple

    let random_tuple = ("Here is a name", 8, vec!['a'], 'b', [8, 9, 10], 7.7);
    println!(
        "Inside the tuple is: First item: {:?}
Second item: {:?}
Third item: {:?}
Fourth item: {:?}
Fifth item: {:?}
Sixth item: {:?}",
        random_tuple.0,
        random_tuple.1,
        random_tuple.2,
        random_tuple.3,
        random_tuple.4,
        random_tuple.5,
    );

    // Vec<(&str, i32)>
    let my_vec = vec![("Hey", 9), ("Hello", 89)];

    // Destructuring
    // Structure

    let str_tuple = ("one", "two", "three");
    let (a, _, _) = str_tuple;

    println!("Item a is : {}", a);
}
