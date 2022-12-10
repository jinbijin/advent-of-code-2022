pub struct VectorChunks<const N: usize, T, U>
where
    U: Iterator<Item = T>,
{
    pub iterator: U,
}

impl<const N: usize, T, U> Iterator for VectorChunks<N, T, U>
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

pub trait AsVectorChunks<T, U>
where
    U: Iterator<Item = T>,
{
    fn vector_chunks<const N: usize>(self) -> VectorChunks<N, T, U>;
}

impl<T, U> AsVectorChunks<T, U> for U
where
    U: Iterator<Item = T>,
{
    fn vector_chunks<const N: usize>(self) -> VectorChunks<N, T, U> {
        VectorChunks { iterator: self }
    }
}
