use std::io;

fn read(input: String) -> String {
    return input;
}

fn eval(input: String) -> String {
    return input;
}

fn print(input: String) -> String {
    return input;
}

fn rep(input: String) -> String
{
    let ast: String = read(input);
    let result: String = eval(ast);
    let output: String = print(result);

    return output;
}

fn main() -> io::Result<()> {
    loop {
        println!("user> ");

        let mut input = String::new();
        let byte_size = io::stdin().read_line(&mut input).unwrap();

        println!("{}\n", rep(input));
    }
}
