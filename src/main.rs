fn main() {
    let func = camel_case("test case");
    println!("{:?}", func);
}


fn camel_case(str: &str) -> String {

    let c : Vec<String> = str
        .split_whitespace()
        .map(|s| format!("{}{}", s[0..1].to_string().to_uppercase(), s[1..s.len()].to_string()) )
        .map(String::from)
        .collect();

    println!("{:?}", c);

    c.join("")
}
