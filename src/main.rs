use std::ops::Index;

fn main() {
    let func = high("man i need a taxi up to ubud");
    println!("{:?}", func);
}


fn high(input: &str) -> &str {
    let dd: Vec<&str> = input.split(" ").collect();
    let mut n: Vec<i32> = Vec::new();
    for d in dd {
        let mut sum = 0;
        for i in d.chars() {
            sum += i as i32;
        }
        n.push(sum)
    }
    let mut ma = n[0];
    for i in &n {
        if ma < *i {
            ma = *i;
        }
    }

    // for i in n.len() {
    //     println!("{}", n[i]);
    // }

    // let index: i32 = n.index(ma);

    // println!("{:?}", index);
    println!("{:?}", n);

    ""
}
