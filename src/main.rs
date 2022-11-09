fn main() {
    let func = count_positives_sum_negatives(vec![]);

    println!("{:?}", func);
}
fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    if input.len() == 0 {
       return  input
    }
   let pos =  *input.iter().max().unwrap();
    let neg =  input.into_iter().filter(|&a| a < 0).reduce(|x, y| x + y).unwrap();

    vec![pos, neg]

}
