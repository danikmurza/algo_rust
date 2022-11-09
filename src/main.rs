use std::mem::transmute;

fn main() {
    let func = get_count("abracadabra");

    println!("{:?}", func);
}
fn get_count(string: &str) -> usize {


    string.matches(|x| match x { 'a'|'e'|'i'|'o'|'u' => true, _=> false }).count()


}
