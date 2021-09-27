use crate::Hunk;

type HunkReader<T> = fn(&Hunk)->T;

// ===========================================================
// Hunk Ref
// ===========================================================

pub struct Ref<'a,T> {
    hunk : &'a Hunk,
    // Item Reader
    reader: HunkReader<T>
}

impl<'a,T> Ref<'a,T> {
    pub fn new(hunk: &'a Hunk, reader: HunkReader<T>) -> Ref<'a,T> {
	Ref{hunk,reader}
    }

    pub fn get(&self) -> T {
	(self.reader)(self.hunk)
    }
}
