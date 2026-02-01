use std::i32;

fn main() {
    let mut map: u32 = 0b11111111111111111111111111111111;
    println!("{:b}", map);

    // let remove_index = 3;
    // map &= !(1 << remove_index);

    // println!("{:b}", map);

    // // input data 1st
    let found_index = map.trailing_ones();
    println!("index di temukan {}", found_index);
    // input data {...}
    map |= 1 << found_index;

    println!("{:b}", map);

    // // input data 2nd
    // let found_index = map.trailing_ones();
    // // input data {...}
    // map |= 1 << found_index;

    // println!("{:b}", map);

    // // input data 3rd
    // let found_index = map.trailing_ones();
    // // input data {...}
    // map |= 1 << found_index;

    // println!("{}", 0b110_i32.leading_zeros());
}
