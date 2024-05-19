use crate::Buf;

/// Iterator over the bytes contained by the buffer.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use bytes::Bytes;
///
/// let buf = Bytes::from(&b"abc"[..]);
/// let mut iter = buf.into_iter();
///
/// assert_eq!(iter.next(), Some(b'a'));
/// assert_eq!(iter.next(), Some(b'b'));
/// assert_eq!(iter.next(), Some(b'c'));
/// assert_eq!(iter.next(), None);
/// ```
#[derive(Debug)]
pub struct IntoIter<T> {
    inner: T,
}

impl<T> IntoIter<T> {
    /// Creates an iterator over the bytes contained by the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Bytes;
    ///
    /// let buf = Bytes::from_static(b"abc");
    /// let mut iter = buf.into_iter();
    ///
    /// assert_eq!(iter.next(), Some(b'a'));
    /// assert_eq!(iter.next(), Some(b'b'));
    /// assert_eq!(iter.next(), Some(b'c'));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn new(inner: T) -> IntoIter<T> {
        IntoIter { inner }
    }

    /// Consumes this `IntoIter`, returning the underlying value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytes::{Buf, Bytes};
    ///
    /// let buf = Bytes::from(&b"abc"[..]);
    /// let mut iter = buf.into_iter();
    ///
    /// assert_eq!(iter.next(), Some(b'a'));
    ///
    /// let buf = iter.into_inner();
    /// assert_eq!(2, buf.remaining());
    /// ```
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Gets a reference to the underlying `Buf`.
    ///
    /// It is inadvisable to directly read from the underlying `Buf`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytes::{Buf, Bytes};
    ///
    /// let buf = Bytes::from(&b"abc"[..]);
    /// let mut iter = buf.into_iter();
    ///
    /// assert_eq!(iter.next(), Some(b'a'));
    ///
    /// assert_eq!(2, iter.get_ref().remaining());
    /// ```
    pub fn get_ref(&self) -> &T {
        &self.inner
    }

    /// Gets a mutable reference to the underlying `Buf`.
    ///
    /// It is inadvisable to directly read from the underlying `Buf`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytes::{Buf, BytesMut};
    ///
    /// let buf = BytesMut::from(&b"abc"[..]);
    /// let mut iter = buf.into_iter();
    ///
    /// assert_eq!(iter.next(), Some(b'a'));
    ///
    /// iter.get_mut().advance(1);
    ///
    /// assert_eq!(iter.next(), Some(b'c'));
    /// ```
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T: Buf> Iterator for IntoIter<T> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        let b = *self.inner.chunk().get(0)?;
        self.inner.advance(1);

        Some(b)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let rem = self.inner.remaining();
        (rem, Some(rem))
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.inner.remaining()
    }

    fn fold<B, F>(mut self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        let mut accum = init;
        loop {
            let chunk = self.inner.chunk();
            if chunk.is_empty() {
                break;
            }

            accum = chunk.iter().copied().fold(accum, &mut f);
            self.inner.advance(chunk.len());
        }

        accum
    }

    fn last(mut self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.inner.advance(self.inner.remaining().checked_sub(1)?);
        Some(self.inner.chunk()[0])
    }
}

impl<T: Buf> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        self.inner.remaining()
    }
}
