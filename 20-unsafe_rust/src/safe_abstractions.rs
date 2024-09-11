use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
  let len = values.len();
  let ptr = values.as_mut_ptr();

  assert!(mid <= len);

  unsafe {
    (
      slice::from_raw_parts_mut(ptr, mid),
      slice::from_raw_parts_mut(ptr.add(mid), len - mid),
    )
  }
}

pub fn run() {
  println!("\nSAFE ABSTRACTIONS OVER UNSAFE CODE:");

  let mut v = vec![1, 2, 3, 4, 5, 6];
  println!("Original slice: {v:?}");

  let (former, latter) = split_at_mut(&mut v, 3);
  println!("Split slices: {former:?} {latter:?}");
}
