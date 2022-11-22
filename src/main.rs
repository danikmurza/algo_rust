

fn main() {

    let aa: Vec<String> = vec!["alice and bob love leetcode".to_string(),"i think so too".to_string(),"this is great thanks very much".to_string()];
    let func = most_words_found(aa);
    println!("{:?}", func);
}

fn most_words_found(sentences: Vec<String>) -> i32 {



    let mut a = sentences.iter().map(|a| a.split_whitespace().count() as i32).collect::<Vec<i32>>();
    a.sort();
    a[a.len()-1]

}

