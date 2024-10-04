/*
** src/itertools.rs
*/

pub struct PairIterator<I> {
    iter: I,
}

impl<I> PairIterator<I> {
    pub fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<'a, I, T> Iterator for PairIterator<I>
where
    T: 'a,
    I: Iterator<Item = &'a T>,
{
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.iter.next();
        let y = self.iter.next();
        if let (Some(a), Some(b)) = (x, y) {
            Some((a, b))
        } else {
            None
        }
    }
}

pub trait Pair<T>: Iterator<Item = T> + Sized {
    fn paired(self) -> PairIterator<Self> {
        PairIterator::new(self)
    }
}

impl<T, I: Iterator<Item = T>> Pair<T> for I {}
