use std::{cmp, collections::HashSet, fmt::Debug, hash::Hash, iter::Sum, ops::Sub};

pub enum BuildIntervalError<T>
where
    T: Debug + Copy + Ord + Hash + Sub<T, Output = T> + Sum,
{
    EndBeforeStart { start: T, end: T },
}

/// A range with non-inclusive end
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Interval<T>
where
    T: Debug + Copy + Hash + Ord + Sub<T, Output = T> + Sum,
{
    start: T,
    end: T,
}

impl<T> Interval<T>
where
    T: Debug + Copy + Hash + Ord + Sub<T, Output = T> + Sum,
{
    pub fn build(start: T, end: T) -> Result<Self, BuildIntervalError<T>> {
        if end < start {
            Err(BuildIntervalError::EndBeforeStart { start, end })
        } else {
            Ok(Interval { start, end })
        }
    }

    pub fn overlap(&self, other: Interval<T>) -> Option<Interval<T>> {
        if self.start <= other.end && self.end >= other.start {
            Some(Interval {
                start: cmp::max(self.start, other.start),
                end: cmp::min(self.end, other.end),
            })
        } else {
            None
        }
    }

    pub fn start(&self) -> T {
        self.start
    }

    pub fn end(&self) -> T {
        self.end
    }

    pub fn count(&self) -> T {
        self.end - self.start
    }

    pub fn contains(&self, point: T) -> bool {
        self.start <= point && point < self.end
    }
}

#[derive(Debug, Clone)]
pub struct IntervalUnion<T>(pub HashSet<Interval<T>>)
where
    T: Debug + Copy + Hash + Ord + Sub<T, Output = T> + Sum;

impl<T> IntervalUnion<T>
where
    T: Debug + Copy + Hash + Ord + Sub<T, Output = T> + Sum,
{
    pub fn new() -> IntervalUnion<T> {
        IntervalUnion(HashSet::new())
    }

    pub fn add(&mut self, interval: &Interval<T>) {
        let IntervalUnion(set) = self;
        let overlapping = set
            .iter()
            .filter(|x| interval.overlap(**x).is_some())
            .map(|x| *x)
            .collect::<Vec<Interval<T>>>();
        let start = overlapping.iter().map(|x| x.start).min();
        let end = overlapping.iter().map(|x| x.end).max();
        for x in overlapping {
            set.remove(&x);
        }
        if let Some(start) = start {
            if let Some(end) = end {
                set.insert(Interval {
                    start: cmp::min(start, interval.start),
                    end: cmp::max(end, interval.end),
                });
            } else {
                set.insert(*interval);
            }
        } else {
            set.insert(*interval);
        }
    }

    pub fn count(&self) -> T {
        self.0.iter().map(|x| x.count()).sum()
    }

    pub fn contains(&self, point: T) -> bool {
        self.0.iter().any(|x| x.contains(point))
    }

    pub fn overlap(&self, interval: Interval<T>) -> IntervalUnion<T> {
        let mut union: IntervalUnion<T> = IntervalUnion::new();

        for self_interval in self.0.iter() {
            if let Some(overlap) = self_interval.overlap(interval) {
                if overlap.start != overlap.end {
                    union.add(&overlap);
                }
            }
        }

        union
    }
}
