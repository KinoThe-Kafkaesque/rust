// fn main() {
//     println!("Hello, world!");

//     another_function();
// }

// fn another_function() {
//     println!("Another function.");
// }


fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
fn five() -> i32 {
    
    return 5
}


fn main() {
    print_labeled_measurement(5, 'h');
    another_function(5);
    let x = five();
    println!("The value of x is: {x}");
}
