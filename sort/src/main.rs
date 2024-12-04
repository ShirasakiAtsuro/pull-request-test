fn main() {
    let mut number = vec![1, 5, 2, 4, 3];

    sort(&mut number);

    println!("{:?}", number);
    println!("{:?}", number);
}

fn sort(vec: &mut Vec<i32>) {
    for i in 0..vec.len() {
        let mut tmp = i;
        for j in i..vec.len() {
            if vec[tmp] < vec[j] {
                tmp = j;
            }
        }

        let tp = vec[i];
        vec[i] = vec[tmp];
        vec[tmp] = tp;
    }
}
