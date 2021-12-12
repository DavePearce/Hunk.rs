use crate::Region;

pub struct Rewrite {
    /// Portion of hunk being replaced
    region: Region,
    /// data replacing portion of hunk
    data: Vec<u8>
}

impl Rewrite {
    pub fn new(region: Region, data: &[u8]) -> Rewrite {
	Rewrite{region,data: data.to_vec()}
    }
}
