fn main() {
    let mut v = vec![10, 20, 9, 7, 1];
    let a = selection_sort(&mut v);
    println!("{:?}", a);
}
fn selection_sort(v: &mut Vec<i32>) -> &mut Vec<i32> {
    let n = v.len();
    let mut temp: i32;
    for i in 0..n - 1 {
        let mut min = i;
        for j in i + 1..n {
            if v[j] < v[min] {
                min = j;
            }
        }
        temp = v[min];
        v[min] = v[i];
        v[i] = temp;
    }
    v
}
