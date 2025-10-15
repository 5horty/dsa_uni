fn main() {
    let mut v = vec![0, 18, 9, 7, 0, 15, 7, 18];
    println!("{:?}", v);
    let a = linear(&mut v, 15);
    if let Some(a) = a {
        println!("{:?}", a); // 5
    }
}

fn linear(v: &mut Vec<i32>, target: i32) -> Option<usize> {
    let n = v.len();
    for i in 0..n {
        if v[i] == target {
            return Some(i);
        }
    }
    None
}
