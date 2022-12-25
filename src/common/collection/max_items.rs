use std::collections::BTreeSet;

pub trait MaxItems: Sized {
    fn max_items<I>(iter: I, count: usize) -> Vec<Self>
    where
        I: Iterator<Item = Self>;
}

impl<T> MaxItems for T
where
    T: Ord,
{
    fn max_items<I>(iter: I, count: usize) -> Vec<Self>
    where
        I: Iterator<Item = Self>,
    {
        let mut top_items: BTreeSet<T> = BTreeSet::new();

        for item in iter {
            top_items.insert(item);
            if top_items.len() > count {
                top_items.pop_first();
            }
        }

        top_items.into_iter().rev().collect::<Vec<T>>()
    }
}

pub trait AsMaxItems {
    type Item;

    fn max_items(self, count: usize) -> Vec<Self::Item>;
}

impl<I> AsMaxItems for I
where
    I: Iterator,
    I::Item: MaxItems,
{
    type Item = I::Item;

    fn max_items(self, count: usize) -> Vec<Self::Item> {
        MaxItems::max_items(self, count)
    }
}
