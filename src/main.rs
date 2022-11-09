

fn main() {
    let func = algo(348597);

    println!("{:?}", func);
}


fn algo(num: u64) ->Vec<u8> {
    let mut vec: Vec<u8> = Vec::new();
    let  split: Vec<u8> = num
        .to_string()
        .split("")
        .filter(|a| a != &"")
        .map(|b| b.parse::<u8>().unwrap())
        .collect();
    for &i in split.iter().rev() {
        vec.push(i as u8);
       }
vec
}
