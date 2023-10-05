fn main() {
    let mut i: i32 = 99;
    while i >= 1 {
        if i == 1 {
            println!("{} bottle of beer on the wall,", i);
            println!("{} bottle of beer.", i);
        } else {
            println!("{} bottles of beer on the wall,", i);
            println!("{} bottles of beer.", i);
        }

        println!("Take one down, pass it around,");

        if i - 1 == 1 {
            println!("{} bottle of beer on the wall.", i - 1);
        } else if i - 1 == 0 {
            println!("No bottles of beer on the wall.");
        }

        println!();
        i = i - 1;
    }
}
