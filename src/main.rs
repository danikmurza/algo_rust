fn main() {
    let func = wave("hello");
    println!("{:?}", func);
}


fn wave(s: &str) -> Vec<String> {
    let mut ve: Vec<String> = Vec::new();
    let f = s[..1].to_uppercase();
    let ff = s[1..s.len()].to_string();
    let fff = format!("{}{}", f, ff);
    ve.push(fff);
    for i in 1..s.len() {
        let d = format!("{}{}{}", &s[0..i], &s[i..i + 1].to_uppercase(), &s[i + 1..s.len()]);
        ve.push(d);
    }
    ve
}
