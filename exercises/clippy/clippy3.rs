// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    // if let None=my_option{
        // my_option.unwrap();
        // println!("hwlll");
    // }
    // if  my_option.is_none() {
    //     my_option.unwrap();
    // }
    if let Some(()) = my_option {
        my_option.unwrap();
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    std::mem::swap(&mut value_a, &mut value_b);
    // value_a = value_b;
    // value_b = value_a;
    println!("value a: {}; value b: {}", value_a, value_b);
}
