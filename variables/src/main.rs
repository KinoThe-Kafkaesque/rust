fn main() {
    let mut  x=5;
    println!("the value of x is {x}");
    x= 6;
    println!("the value of x is {x}");
    const THREE_HOURS : i32 = 3*60*60;
    println!("the value of x is {x}");
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
