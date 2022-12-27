use std::{
    cmp,
    collections::HashSet,
    fmt::Debug,
    hash::Hash,
    iter::Sum,
    ops::{Add, Sub},
};

#[derive(Debug)]
pub enum BuildIntervalError<T>
where
    T: Debug + Copy + Ord + Hash + Add<T, Output = T> + Sub<T, Output = T> + Sum,
{
    EndBeforeStart { start: T, end: T },
}

/// A range with non-inclusive end
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Interval<T>
where
    T: Debug + Copy + Hash + Ord + Add<T, Output = T> + Sub<T, Output = T> + Sum,
{
    start: T,
    end: T,
}

impl<T> Interval<T>
where
    T: Debug + Copy + Hash + Ord + Add<T, Output = T> + Sub<T, Output = T> + Sum,
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
    T: Debug + Copy + Hash + Ord + Add<T, Output = T> + Sub<T, Output = T> + Sum;

impl<T> IntervalUnion<T>
where
    T: Debug + Copy + Hash + Ord + Add<T, Output = T> + Sub<T, Output = T> + Sum,
{
    pub fn new() -> IntervalUnion<T> {
        IntervalUnion(HashSet::new())
    }

    pub fn expand(&self, by: T) -> IntervalUnion<T> {
        let IntervalUnion(set) = self;
        let mut new_interval_union: IntervalUnion<T> = IntervalUnion::new();

        for interval in set.iter() {
            new_interval_union.add(&Interval {
                start: interval.start - by,
                end: interval.end + by,
            })
        }

        new_interval_union
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

    pub fn remove(&mut self, other: &IntervalUnion<T>) {
        let IntervalUnion(other) = other;
        for interval in other {
            self.remove_interval(interval);
        }
    }

    fn remove_interval(&mut self, interval: &Interval<T>) {
        let IntervalUnion(set) = self;
        let overlapping = set
            .iter()
            .filter(|x| interval.overlap(**x).is_some())
            .map(|x| *x)
            .collect::<Vec<Interval<T>>>();

        let mut trimmed: Vec<Interval<T>> = Vec::new();
        for x in overlapping.iter() {
            if x.start < interval.start {
                trimmed.push(Interval {
                    start: x.start,
                    end: interval.start,
                });
            }
            if x.end > interval.end {
                trimmed.push(Interval {
                    start: interval.end,
                    end: x.end,
                });
            }
            set.remove(x);
        }

        for new_interval in trimmed {
            set.insert(new_interval);
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
