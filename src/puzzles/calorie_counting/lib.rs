use std::iter::Sum;

struct SplitByNoneIterator<'a, T, U>
where
    T: Sum,
    U: Iterator<Item = Option<T>>,
{
    iterator: &'a mut U,
}

impl<'a, T, U> Iterator for SplitByNoneIterator<'a, T, U>
where
    T: Sum,
    U: Iterator<Item = Option<T>>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut end_of_iterator = true;
        let result = self
            .iterator
            .by_ref()
            .take_while(|item| item.is_some())
            .filter_map(|item| {
                end_of_iterator = false;
                item
            })
            .sum::<T>();
        if end_of_iterator {
            None
        } else {
            Some(result)
        }
    }
}

pub fn sum_of_top_group_sums<T>(calories: &mut impl Iterator<Item = Option<T>>, count: usize) -> T
where
    T: Sum + Ord,
{
    let elf_iterator = SplitByNoneIterator { iterator: calories };
    let mut elf_vector = elf_iterator.collect::<Vec<T>>();
    elf_vector.sort_by(|a, b| b.cmp(a));
    elf_vector.into_iter().take(count).sum()
}
