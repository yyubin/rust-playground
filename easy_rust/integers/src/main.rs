fn main() {
    // i8, i16, i32, i64, i128 and isize.
    // u8, u16, u32, u64, u128 and usize.
    // 8 bits = 1 byte

    // isize -> 32bit -> i32
    // isize -> 64bit -> i64

    let my_number: u8 = 100; // 타입 지정안할시 i32 
    let my_other_number = 50; // i32
    let third_number = my_number + my_other_number;
    // u8 + i32 => 계산 불가능
    // i32로 지정되지 않고 my_other_number도 u8타입으로 지정

    // type inference
}
