

fn main() {
    let func = num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string());
    println!("{:?}", func);
}

fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {

    let j: Vec<char> = jewels.chars().collect::<Vec<char>>();
    let s: Vec<char> = stones.chars().collect::<Vec<char>>();

    let mut count  = 0;
    for i in 0..s.len() {
        for k in 0..j.len() {
            if s[i] == j[k] {
                count = count +1;
            }
        }
    }
    count
}

