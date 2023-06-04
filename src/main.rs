const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();

    println!("Number of spaces: {spaces}");

    println!("Three hours in secs: {THREE_HOURS_IN_SECONDS}");
}
