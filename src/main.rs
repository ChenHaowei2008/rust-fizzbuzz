fn main() {
    let mut output: String;
    for n in 1..101{
        output = String::default();
        if n % 3 == 0 {
            output = output.to_owned() + "fizz";
        }
        if n % 5 == 0 {
            output = output.to_owned() + "buzz";
        }
        if output == "" {
            output = n.to_string();
        }
        println!("{}", output)
    }
}