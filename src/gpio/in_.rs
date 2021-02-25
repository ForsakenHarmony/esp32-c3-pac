#[doc = "Reader of register IN"]
pub type R = crate::R<u32, super::IN>;
#[doc = "Reader of field `IN_DATA`"]
pub type IN_DATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn in_data(&self) -> IN_DATA_R {
        IN_DATA_R::new((self.bits & 0x03ff_ffff) as u32)
    }
}
