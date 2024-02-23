fn main() {
    let mut v = vec![5, 1, 2, 7, 3, 9, 0, 1, 6];
    v.sort();
    println!("Sorted vector: {:?}", v);

    let len = v.len();
    let mid = len / 2;
    let median = if len % 2 == 0 {
        (v[mid - 1] + v[mid]) / 2
    } else {
        v[mid]
    };

    println!("Median: {}", median);

    let mut map = std::collections::HashMap::new();
    for i in &v {
        let count: &mut i32 = map.entry(i).or_insert(0);
        *count += 1;
    }

    let mut max_count = 0;
    let mut mode = 0;
    for (k, v) in &map {
        if max_count < *v {
            max_count = *v;
            mode = **k;
        }
    }

    println!("Mode: {}", mode);
}