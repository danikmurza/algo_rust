fn main() {
    let func = high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4");

    println!("{:?}", func);
}
fn high_and_low(numbers: &str) -> String {
    let mut st: Vec<i32> = numbers.split(" ").map(|n| n.parse::<i32>().unwrap()).collect();
    st.sort();
    format!("{} {}", st[st.len() - 1], st[0])
}
