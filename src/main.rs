
fn main() {
    let mut output: String = String::new();
    let mut n:u32 = 0;
    while n < 101{
        output = String::default();
        n += 1;
        if n % 3 == 0 {
            output = output.to_owned() + "fizz";
        }
        if n % 5 == 0 {
            output = output.to_owned() + "buzz";
        }
        if output == "" {
            output  = n.to_string();
        }
        println!("{}", output)
    }
}