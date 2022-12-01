use std::iter::Sum;

struct SplitByNoneIterator<'a, T, U> where T : Sum, U : Iterator<Item = Option<T>> {
    iterator: &'a mut U
}

impl<'a, T, U> Iterator for SplitByNoneIterator<'a, T, U> where T : Sum, U : Iterator<Item = Option<T>> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut end_of_iterator = true;
        let result = self.iterator
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

pub fn sum_of_top_group_sums<T>(
    calories: &mut impl Iterator<Item = Option<T>>,
    count: usize,
) -> T
where
    T: Sum + Ord,
{
    let elf_iterator = SplitByNoneIterator {
        iterator: calories
    };
    let mut elf_vector = elf_iterator.collect::<Vec<T>>();
    elf_vector.sort_by(|a, b| b.cmp(a));
    elf_vector.into_iter().take(count).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn example_part_1_should_be_correct() -> Result<(), Box<dyn Error>> {
        let calories = vec![
            Some(1000),
            Some(2000),
            Some(3000),
            None,
            Some(4000),
            None,
            Some(5000),
            Some(6000),
            None,
            Some(7000),
            Some(8000),
            Some(9000),
            None,
            Some(10000),
        ];

        let result = sum_of_top_group_sums::<i32>(&mut calories.into_iter(), 1);

        assert_eq!(24000, result);

        Ok(())
    }

    #[test]
    fn example_part_2_should_be_correct() -> Result<(), Box<dyn Error>> {
        let calories = vec![
            Some(1000),
            Some(2000),
            Some(3000),
            None,
            Some(4000),
            None,
            Some(5000),
            Some(6000),
            None,
            Some(7000),
            Some(8000),
            Some(9000),
            None,
            Some(10000),
        ];

        let result = sum_of_top_group_sums::<i32>(&mut calories.into_iter(), 3);

        assert_eq!(45000, result);

        Ok(())
    }
}
