pub trait BitBoardIdx: Copy {
    fn idx(self) -> u64;

    fn idx_usize(self) -> usize
    where
        Self: Sized,
    {
        self.idx() as usize
    }
}

impl BitBoardIdx for (u64, u64) {
    fn idx(self) -> u64 {
        self.0 * 8 + self.1
    }
}

impl BitBoardIdx for (usize, usize) {
    fn idx(self) -> u64 {
        (self.0 as u64) * 8 + (self.1 as u64)
    }
}

impl BitBoardIdx for u64 {
    fn idx(self) -> u64 {
        self
    }
}

impl BitBoardIdx for usize {
    fn idx(self) -> u64 {
        self as u64
    }
}

impl BitBoardIdx for i32 {
    fn idx(self) -> u64 {
        self as u64
    }
}
