fn main() {
    let mut v = vec![10, 3, 7, 8, 3, 4];
    println!("{v:?}");
    bubble_sort(&mut v);
    println!("{v:?}");
}

fn bubble_sort(v: &mut Vec<i32>) -> &mut Vec<i32> {
    let n = v.len();
    for i in 0..n {
        for j in i + 1..n {
            if v[j] < v[i] {
                v.swap(i, j);
            }
        }
    }
    v
}
