use std::slice;

fn split_at_mut_custom(values: &mut [i32],
                       mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len-mid)
        )
    }
}

fn main() {
    let mut v = vec![1, 4, 2, 5, 3, 6];
    let r = &mut v[..];

    // split_at_mut uses unsafe code wrapped in a safe API
    let (a, b) = split_at_mut_custom(r, 3); 

    assert_eq!(a, &mut [1, 4, 2]);
    assert_eq!(b, &mut [5, 3, 6]);
    print!("A = ");
    for i in a{
        print!("{} ", i);
    }
    println!();
    print!("B = ");
    for i in b{
        print!("{} ", i);
    }
    println!();
}
