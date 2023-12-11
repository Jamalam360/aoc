/// The position of iter.max()
#[inline(always)]
pub fn index_of_max<I>(mut iter: impl Iterator<Item = I>) -> Option<usize>
where
    I: Ord,
{
    let mut i = 0usize;
    let mut max = None;

    while let Some(next) = iter.next() {
        if let Some((_, prev_max)) = &max {
            if &next > prev_max {
                max = Some((i, next))
            }
        } else {
            max = Some((i, next));
        }

        i += 1;
    }

    max.map(|i| i.0)
}

pub struct Difference<I>
where
    I: Iterator,
    I::Item: std::ops::Sub<Output = I::Item> + Clone + Copy,
{
    underlying: I,
    previous: Option<I::Item>,
}

impl<I> Difference<I>
where
    I: Iterator,
    I::Item: std::ops::Sub<Output = I::Item> + Clone + Copy,
{
    fn new(mut iterator: I) -> Self {
        Difference {
            previous: iterator.next(),
            underlying: iterator,
        }
    }
}

impl<I> Iterator for Difference<I>
where
    I: Iterator,
    I::Item: std::ops::Sub<Output = I::Item> + Clone + Copy,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let next_item = self.underlying.next()?;

        let diff = match self.previous.take() {
            Some(prev) => Some(next_item - prev),
            None => None,
        };

        self.previous = Some(next_item);
        Some(diff).flatten()
    }
}

pub trait DifferenceExt: Iterator {
    /// Returns the differences between adjacent pairs of items in the iterator
    fn differences(self) -> Difference<Self>
    where
        Self: Sized,
        Self::Item: std::ops::Sub<Output = Self::Item> + Clone + Copy,
    {
        Difference::new(self)
    }
}

impl<I: Iterator> DifferenceExt for I {}
