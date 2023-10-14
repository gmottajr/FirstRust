// variables

fn main() {
    let x: u32 = 5;
    let y: u32 = 10;
    let z = x + y;
    println!("***********************************************************");
    println!("");
    println!("The sum of x and y is {}", z);
    println!("Hello, world!");
    println!("");
    println!("By default all variables in Rust are immutable");
    println!("-------------------------- creating a mutable variable  --------------------------");
    let mut x = 43;
    println!("let mut x = 43;");
    x = x + 1;
    println!("x = {}", x);

    // You can assign multible variables at once!
    let (name, feature, hoooby) = ("Israel", "smart", "guitar");
    println!("My name is {}, I'm {}! I love playing {}!!!", name, feature, hoooby);

    // Constants
    const Id: i32 = 1023;
    const Name: &str = "Gerson";
    println!("User id: {} - Name: {}", Id, Name);
    println!("***********************************************************");
    
}
