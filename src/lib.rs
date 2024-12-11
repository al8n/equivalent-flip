// These traits are based on `equivalent` crate, but `K` and `Q` are flipped to avoid type inference issues:
// https://github.com/indexmap-rs/equivalent/issues/5

#![doc = include_str!("../README.md")]
#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![allow(rustdoc::bare_urls)]
#![deny(missing_docs)]

#[cfg(test)]
extern crate std;

use core::{borrow::Borrow, cmp::Ordering};

/// Key equivalence trait.
///
/// This trait allows hash table lookup to be customized. It has one blanket
/// implementation that uses the regular solution with `Borrow` and `Eq`, just
/// like `HashMap` does, so that you can pass `&str` to lookup into a map with
/// `String` keys and so on.
///
/// # Contract
///
/// The implementor **must** hash like `Q`, if it is hashable.
pub trait Equivalent<Q: ?Sized> {
  /// Compare self to `key` and return `true` if they are equal.
  fn equivalent(&self, key: &Q) -> bool;
}

impl<K: ?Sized, Q: ?Sized> Equivalent<Q> for K
where
  K: Borrow<Q>,
  Q: Eq,
{
  #[inline]
  fn equivalent(&self, key: &Q) -> bool {
    PartialEq::eq(self.borrow(), key)
  }
}

/// Key ordering trait.
///
/// This trait allows ordered map lookup to be customized. It has one blanket
/// implementation that uses the regular solution with `Borrow` and `Ord`, just
/// like `BTreeMap` does, so that you can pass `&str` to lookup into a map with
/// `String` keys and so on.
pub trait Comparable<Q: ?Sized>: Equivalent<Q> {
  /// Compare self to `key` and return their ordering.
  fn compare(&self, key: &Q) -> Ordering;
}

impl<K: ?Sized, Q: ?Sized> Comparable<Q> for K
where
  K: Borrow<Q>,
  Q: Ord,
{
  #[inline]
  fn compare(&self, key: &Q) -> Ordering {
    Ord::cmp(self.borrow(), key)
  }
}

/// `ComparableRangeBounds` is implemented as an extention to `RangeBounds` to
/// allow for comparison of items with range bounds.
pub trait ComparableRangeBounds<Q: ?Sized>: core::ops::RangeBounds<Q> {
  /// Returns `true` if `item` is contained in the range.
  fn compare_contains<K>(&self, item: &K) -> bool
  where
    K: ?Sized + Comparable<Q>,
  {
    use core::ops::Bound;

    (match self.start_bound() {
      Bound::Included(start) => item.compare(start) != Ordering::Less,
      Bound::Excluded(start) => item.compare(start) == Ordering::Greater,
      Bound::Unbounded => true,
    }) && (match self.end_bound() {
      Bound::Included(end) => item.compare(end) != Ordering::Greater,
      Bound::Excluded(end) => item.compare(end) == Ordering::Less,
      Bound::Unbounded => true,
    })
  }
}

impl<R, Q> ComparableRangeBounds<Q> for R
where
  R: ?Sized + core::ops::RangeBounds<Q>,
  Q: ?Sized,
{
}
