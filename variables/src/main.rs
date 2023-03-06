
fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Number of seconds in 3 hours: {THREE_HOURS_IN_SECONDS}");

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of mutated x is: {x}");

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of shadowed x after scope is: {x}");

    let spaces = "    ";
    let spaces = spaces.len();
    println!("Spaces length: {spaces}");
}
