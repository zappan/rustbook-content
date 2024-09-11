#[cfg(test)]
mod test {

  #[test]
  fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    // We need to make v1_iter mutable: calling the next method on an iterator changes
    // internal state that the iterator uses to keep track of where it is in the sequence.
    // In other words, this code consumes, or uses up, the iterator. Each call to next()
    // eats up an item from the iterator.
    let mut v1_iter = v1.iter();

    // We didn’t need to make v1_iter mutable when we used a for loop (see main.rs) because
    // the loop took ownership of v1_iter and made it mutable behind the scenes.

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
  }

  #[test]
  // Methods that call next (internally) are called consuming adaptors, because calling them
  // uses up the iterator. One example is the sum method, which takes ownership of the iterator
  // and iterates through the items by repeatedly calling next, thus consuming the iterator.
  fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    // We aren’t allowed to use v1_iter after the call to sum()
    // because sum() takes ownership of the iterator we call it on.
    assert_eq!(total, 6);
  }
}
