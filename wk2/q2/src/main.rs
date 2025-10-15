fn main() {
    let mut v = vec![10, 3, 7, 2, 5, 90];
    insert_sort(&mut v);
    println!("{:?}", v);
}
fn insert_sort(v: &mut Vec<i32>) {
    let n = v.len();
    for i in 1..n {
        let mut cur = v[i];
        let mut j = i;
        while j > 0 && cur < v[j - 1] {
            v[j] = v[j - 1];
            j -= 1;
        }
        v[j] = cur;
    }
}
