pub trait Increment {
    fn increment(&self) -> Self;
}

pub trait Decrement {
    fn decrement(&self) -> Self;
}

impl Increment for usize {
    fn increment(&self) -> Self {
        self + 1
    }
}

impl Increment for isize {
    fn increment(&self) -> Self {
        self + 1
    }
}

impl Decrement for usize {
    fn decrement(&self) -> Self {
        self - 1
    }
}

impl Decrement for isize {
    fn decrement(&self) -> Self {
        self - 1
    }
}
