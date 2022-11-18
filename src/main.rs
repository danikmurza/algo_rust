fn main() {
    let func = find_outlier(&[160, 3, 1719, 19, 11, 13, -21]);
    println!("{:?}", func);
}


fn find_outlier(values: &[i32]) -> &i32 {
    let a = values.into_iter().filter(|&&x| x % 2 == 0).collect::<Vec<&i32>>();
    let bb = values.into_iter().filter(|&&x| x % 2 == 1).collect::<Vec<&i32>>();
    return if a.len() > bb.len() {
        &bb[0]
    } else {
        a[0]
    };
}
