use crate::Hunk;

type HunkReader<T> = fn(&dyn Hunk)->T;

// ===========================================================
// Hunk Ref
// ===========================================================

pub struct Ref<'a,T> {
    hunk : &'a dyn Hunk,
    // Item Reader
    reader: HunkReader<T>
}

impl<'a,T> Ref<'a,T> {
    pub fn new(hunk: &'a dyn Hunk, reader: HunkReader<T>) -> Ref<'a,T> {
	Ref{hunk,reader}
    }

    pub fn get(&self) -> T {
	(self.reader)(self.hunk)
    }
}
