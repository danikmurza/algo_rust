

fn main() {
    let func = maskify("4556364607935616");
    println!("{:?}", func);
}


fn maskify(cc: &str) -> String {

    if cc.len() < 5 {
       return  cc.to_string()
    }
    "#".repeat(cc.len()-4).to_string() + &cc[cc.len() -4..]

}
