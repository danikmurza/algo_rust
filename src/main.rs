fn main() {
    let func = descending_order(123456789);
    println!("{:?}", func);
}
fn descending_order(x: u64) -> u64 {
   let mut n = x.to_string().chars().collect::<Vec<char>>();
    n.sort_by(|x, y| y.cmp(x));

    String::from_iter(n).parse::<u64>().unwrap()



}
