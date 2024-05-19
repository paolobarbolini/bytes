#![warn(rust_2018_idioms)]

use bytes::{buf::IntoIter, Bytes};

#[test]
fn iter() {
    let buf = Bytes::from_static(b"hello world");
    let mut iter = IntoIter::new(buf);

    let mut items = Vec::new();
    while let Some(item) = iter.next() {
        items.push(item);
    }

    assert_eq!(items, b"hello world");
}

#[test]
fn iter_fold() {
    let buf = Bytes::from_static(b"hello world");
    let iter = IntoIter::new(buf);

    let items = iter.fold(Vec::new(), |mut accum, b| {
        accum.push(b);
        accum
    });

    assert_eq!(items, b"hello world");
}

#[test]
fn iter_len() {
    let buf = Bytes::from_static(b"hello world");
    let iter = buf.iter();

    assert_eq!(iter.size_hint(), (11, Some(11)));
    assert_eq!(iter.len(), 11);
    assert_eq!(iter.count(), 11);
}

#[test]
fn empty_iter_len() {
    let buf = Bytes::from_static(b"");
    let iter = buf.iter();

    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.len(), 0);
    assert_eq!(iter.count(), 0);
}

#[test]
fn iter_last() {
    let buf = Bytes::from_static(b"hello world");
    let iter = IntoIter::new(buf);

    assert_eq!(iter.last(), Some(b'd'));
}

#[test]
fn empty_iter_last() {
    let buf = Bytes::new();
    let iter = IntoIter::new(buf);

    assert_eq!(iter.last(), None);
}
