

pub const FIFO_SIZE: usize = 64;
pub struct Fifo<T>
    where
        T: Copy,
{
    pub storage: [T; FIFO_SIZE],
    pub write: usize,
    pub read: usize,
}

impl<T> Fifo<T>
    where
        T: Copy,
{
    pub fn push(&mut self, data: T) {
        let write_idx = self.write as usize % self.storage.len();
        self.storage[write_idx] = data;
        self.write = self.write.wrapping_add(1);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.read == self.write {
            None
        } else {
            let read_idx = self.read as usize % self.storage.len();
            let data = self.storage[read_idx];
            self.read = self.read.wrapping_add(1);
            Some(data)
        }
    }
}