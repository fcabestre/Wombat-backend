use guid_create::GUID;
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct HGuid(GUID);

impl HGuid {
    pub fn new() -> HGuid {
        HGuid(GUID::rand())
    }
}

impl Hash for HGuid {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.0.data1());
        state.write_u16(self.0.data2());
        state.write_u16(self.0.data3());
        for b in self.0.data4().iter() {
            state.write_u8(*b)
        }
    }
}
