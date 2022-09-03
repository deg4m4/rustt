use std::{str::FromStr, fmt::Display};

fn main() {

    let h = p("Hello, Pka!");
    let f = p(232);
    let s = p("1234".to_string());

}

fn p<T>(n: T) -> T
where T: ToString
{

    let p: String = n.to_string();

    println!("{}", p);

    n

}
