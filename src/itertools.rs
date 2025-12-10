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

pub struct TripleIterator<I> {
    iter: I,
}

impl<I> TripleIterator<I> {
    pub fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<'a, I, T> Iterator for TripleIterator<I>
where
    T: 'a,
    I: Iterator<Item = &'a T>,
{
    type Item = (&'a T, &'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.iter.next();
        let y = self.iter.next();
        let z = self.iter.next();
        if let (Some(a), Some(b), Some(c)) = (x, y, z) {
            Some((a, b, c))
        } else {
            None
        }
    }
}

pub trait Triples<T>: Iterator<Item = T> + Sized {
    fn triples(self) -> TripleIterator<Self> {
        TripleIterator::new(self)
    }
}

impl<T, I: Iterator<Item = T>> Triples<T> for I {}

pub struct DedupIterator<I, T> {
    iter: I,
    previous: Option<T>,
}

impl<I, T> DedupIterator<I, T> {
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            previous: None,
        }
    }
}

impl<I, T> Iterator for DedupIterator<I, T>
where
    T: Clone + PartialEq,
    I: Iterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut item = self.iter.next();
        while item == self.previous {
            item = self.iter.next();
        }
        self.previous = item.clone();
        item
    }
}

pub trait Dedup<T>: Iterator<Item = T> + Sized {
    fn dedup(self) -> DedupIterator<Self, T> {
        DedupIterator::new(self)
    }
}

impl<T, I: Iterator<Item = T>> Dedup<T> for I {}

pub struct CombinationIterator<T>
where
    T: Clone,
{
    a: Vec<T>,
    c: Vec<T>,
    p: Vec<i32>,
    x: i32,
    y: i32,
    z: i32,
    done: bool,
}

impl<T> CombinationIterator<T>
where
    T: Clone,
{
    fn init_twiddle(n: usize, m: usize) -> Vec<i32> {
        let mut p = vec![0; n + 2];
        p[0] = n as i32 + 1;
        p[n + 1] = -2;
        for (i, pp) in p[(n - m + 1)..(n + 1)].iter_mut().enumerate() {
            *pp = i as i32 + 1;
        }
        if m == 0 {
            p[1] = 1;
        }
        p
    }

    pub fn new<I>(iter: I, m: usize) -> Self
    where
        I: Iterator<Item = T>,
    {
        let a = iter.collect::<Vec<_>>();
        let c = a[(a.len() - m)..].to_vec();
        let p = Self::init_twiddle(a.len(), m);
        Self {
            a,
            c,
            p,
            x: 0,
            y: 0,
            z: 0,
            done: false,
        }
    }

    fn twiddle(&mut self) {
        let mut j = 1;
        while self.p[j] <= 0 {
            j += 1;
        }
        if self.p[j - 1] == 0 {
            for i in (1..j).rev() {
                self.p[i] = -1;
            }
            self.p[j] = 0;
            self.p[1] = 1;
            self.x = 0;
            self.y = j as i32 - 1;
            self.z = 0;
        } else {
            if j > 1 {
                self.p[j - 1] = 0;
            }
            j += 1;
            while self.p[j] > 0 {
                j += 1;
            }
            let k = j - 1;
            let mut i = j;
            while self.p[i] == 0 {
                self.p[i] = -1;
                i += 1;
            }
            if self.p[i] == -1 {
                self.x = i as i32 - 1;
                self.y = k as i32 - 1;
                self.z = self.p[k] - 1;
                self.p[i] = self.p[k];
                self.p[k] = -1;
            } else if self.p[0] == i as i32 {
                self.done = true;
            } else {
                self.x = j as i32 - 1;
                self.y = i as i32 - 1;
                self.z = self.p[i] - 1;
                self.p[j] = self.p[i];
                self.p[i] = 0;
            }
        }
    }
}

impl<T> Iterator for CombinationIterator<T>
where
    T: Clone,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.c.is_empty() {
            return None;
        } else if self.done {
            let output = self.c.clone();
            self.c.clear();
            return Some(output);
        }

        let output = self.c.clone();
        self.twiddle();
        self.c[self.z as usize] = self.a[self.x as usize].clone();
        Some(output)
    }
}

pub trait Combinations<T: Clone>: Iterator<Item = T> + Sized {
    fn combinations(self, m: usize) -> CombinationIterator<T> {
        CombinationIterator::new(self, m)
    }
}

impl<T: Clone, I: Iterator<Item = T>> Combinations<T> for I {}

pub trait AllEqual<T: PartialEq>: Iterator<Item = T> + Sized {
    fn all_equal(mut self) -> bool {
        if let Some(first) = self.next() {
            self.all(|item| item == first)
        } else {
            true
        }
    }
}

impl<T: PartialEq, I: Iterator<Item = T>> AllEqual<T> for I {}
