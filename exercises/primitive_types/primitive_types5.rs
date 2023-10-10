// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

fn main() {
    let s :&str="Furry McFurson";
    let cat = (s, 3.5);
    let /* your pattern here */(name,age) = cat;
    println!("{}",cat.0);
    println!("{} is {} years old.", name, age);
}
