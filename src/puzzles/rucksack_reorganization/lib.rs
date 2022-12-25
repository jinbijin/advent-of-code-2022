use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    str::FromStr,
    vec::IntoIter,
};

#[derive(Debug)]
pub enum ParseRucksackError {
    InvalidItem,
    UnevenRucksack,
}

impl Display for ParseRucksackError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidItem => write!(f, "invalid item"),
            Self::UnevenRucksack => write!(f, "uneven number of items in rucksack"),
        }
    }
}

impl Error for ParseRucksackError {}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum RucksackItem {
    Item(char),
}

impl RucksackItem {
    pub fn priority(&self) -> i32 {
        let RucksackItem::Item(value) = self;
        if *value >= 'a' && *value <= 'z' {
            *value as i32 - 'a' as i32 + 1
        } else if *value >= 'A' && *value <= 'Z' {
            *value as i32 - 'A' as i32 + 27
        } else {
            unreachable!("Cannot reach here due to parsing rules.")
        }
    }
}

pub trait RucksackItemCollection {
    fn as_collection(&self) -> Vec<RucksackItem>;
}

pub enum RucksackCompartment<'a> {
    Compartment(&'a [RucksackItem]),
}

impl<'a> RucksackItemCollection for RucksackCompartment<'a> {
    fn as_collection(&self) -> Vec<RucksackItem> {
        let Self::Compartment(contents) = self;
        let mut contents = contents.iter().map(|x| *x).collect::<Vec<RucksackItem>>();
        contents.sort();
        contents
    }
}

pub struct Rucksack {
    contents: Vec<RucksackItem>, // Here to take ownership of the data the slices refer to
    first_compartment_size: usize,
}

impl RucksackItemCollection for Rucksack {
    fn as_collection(&self) -> Vec<RucksackItem> {
        let mut contents = self.contents.clone();
        contents.sort();
        contents
    }
}

impl Rucksack {
    pub fn compartments(&self) -> Vec<RucksackCompartment> {
        vec![
            RucksackCompartment::Compartment(&self.contents[0..self.first_compartment_size]),
            RucksackCompartment::Compartment(&self.contents[self.first_compartment_size..]),
        ]
    }
}

impl FromStr for Rucksack {
    type Err = ParseRucksackError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let contents = s
            .chars()
            .map(|c| {
                if (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') {
                    Ok(RucksackItem::Item(c))
                } else {
                    Err(Self::Err::InvalidItem)
                }
            })
            .collect::<Result<Vec<RucksackItem>, Self::Err>>()?;
        let count = contents.len();
        if count % 2 != 0 {
            return Err(Self::Err::UnevenRucksack);
        }
        let first_compartment_size = count / 2;
        Ok(Self {
            contents,
            first_compartment_size,
        })
    }
}

pub struct VectorChunkIterator<'a, const N: usize, T, U>
where
    U: Iterator<Item = T>,
{
    pub iterator: &'a mut U,
}

impl<'a, const N: usize, T, U> Iterator for VectorChunkIterator<'a, N, T, U>
where
    U: Iterator<Item = T>,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.iterator.by_ref().take(N).collect::<Vec<T>>();
        if result.len() == N {
            Some(result)
        } else {
            None
        }
    }
}

pub fn find_common_item<T>(collections: Vec<T>) -> RucksackItem
where
    T: RucksackItemCollection,
{
    let mut iters = collections
        .iter()
        .map(|collection| collection.as_collection().into_iter())
        .collect::<Vec<IntoIter<RucksackItem>>>();
    let mut current = iters
        .iter_mut()
        .map(|iter| iter.next())
        .collect::<Vec<Option<RucksackItem>>>();

    while current.iter().all(|value| value.is_some()) {
        let current_values = current
            .iter()
            .filter_map(|value| value.as_ref())
            .collect::<Vec<&RucksackItem>>();
        let first_value = current_values[0];
        if current_values.iter().all(|value| *value == first_value) {
            return *first_value;
        }

        if let Some(min_value) = current_values.iter().min() {
            let minimal_indices = current_values
                .iter()
                .enumerate()
                .filter(|(_, x)| *x == min_value)
                .map(|(index, _)| index)
                .collect::<Vec<usize>>();
            for index in minimal_indices {
                current[index] = iters[index].next();
            }
        }
    }

    unreachable!("Input data should always have a common item.");
}
