#[test]
fn iterator_demonstration() {
   let v1 = vec![1, 2, 3];

   // mutable because calling .next() changes state of the iterator
   let mut v1_iter = v1.iter();

   assert_eq!(v1_iter.next(), Some(&1));
   assert_eq!(v1_iter.next(), Some(&2));
   assert_eq!(v1_iter.next(), Some(&3));
   assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // sum() is a consuming adaptor, because it calls next
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}
