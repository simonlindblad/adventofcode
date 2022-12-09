pub mod day1;
pub mod day10;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

/// An iterator extension that allows us to group elements of an iterator in batches. Each element
/// out of the new iterator will be a vector of the batch size, or less if we can't fill up a batch
/// with elements from the iterator.
pub trait BatchedIteratorExt: Sized
where
    Self: Iterator,
{
    fn batch(self, count: usize) -> BatchedIterator<Self>;
}

/// The BatchedIterator is the new iterator that we create in order to go through the batches
pub struct BatchedIterator<Iter: Iterator> {
    // We keep track of final state so we don't have to check the provided iterator twice - some
    // iterator implementations may start over on duplicate `next()` calls.
    done: bool,
    count: usize,
    wrapped: Iter,
}

impl<Iter: Iterator> BatchedIterator<Iter> {
    pub fn new(it: Iter, count: usize) -> BatchedIterator<Iter> {
        BatchedIterator {
            done: false,
            wrapped: it,
            count,
        }
    }
}

impl<Iter: Iterator> Iterator for BatchedIterator<Iter> {
    type Item = Vec<Iter::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let batch = (0..self.count)
            .map(|_| self.wrapped.next())
            .take_while(|e| e.is_some())
            .collect::<Option<Vec<_>>>()
            .unwrap(); // No none due to take-while

        if batch.len() < self.count {
            self.done = true;
        }

        if batch.is_empty() {
            None
        } else {
            Some(batch)
        }
    }
}

// Implement the batched iterator extension for all iterators
impl<Iter: Iterator> BatchedIteratorExt for Iter {
    fn batch(self, count: usize) -> BatchedIterator<Self> {
        BatchedIterator::new(self, count)
    }
}
