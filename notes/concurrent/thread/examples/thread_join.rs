use std::thread;

fn merge_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    let mid = len / 2;
    let (left, right) = arr.split_at_mut(mid);

    merge_sort(left);
    merge_sort(right);
    merge(left, right);
}
fn merge_sort_thread(arr: &mut [i32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    let mid = len / 2;
    let (left, right) = arr.split_at_mut(mid);

    thread::scope(|s| {
        s.spawn(|| merge_sort_thread(left));
        s.spawn(|| merge_sort_thread(right));
    });

    merge(left, right);
}
fn merge(l: &mut [i32], r: &mut [i32]) {
    let (mut i, mut j) = (0, 0);
    let mut arr = Vec::with_capacity(l.len() + r.len());
    while i < l.len() && j < r.len() {
        if l[i] <= r[j] {
            arr.push(l[i]);
            i += 1;
        } else {
            arr.push(r[j]);
            j += 1;
        }
    }
    arr.extend_from_slice(&l[i..]);
    arr.extend_from_slice(&r[j..]);
    l.iter_mut()
        .chain(r.iter_mut())
        .zip(arr.into_iter())
        .for_each(|(a, b)| *a = b);
}

fn main() {
    let mut handles = Vec::new();
    for i in 0..5 {
        handles.push(thread::spawn(move || {
            println!("Hello, world! {}", i);
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let mut arr = vec![10, 2, 3, 4, 5, 6, 7, 8, 9, 1];
    let mut arr_thread = arr.clone();
    merge_sort(&mut arr);
    merge_sort_thread(&mut arr_thread);
    println!("{:?}", arr);
    println!("{:?}", arr_thread);
}
