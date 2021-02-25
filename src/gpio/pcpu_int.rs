#[doc = "Reader of register PCPU_INT"]
pub type R = crate::R<u32, super::PCPU_INT>;
#[doc = "Reader of field `PROCPU_INT`"]
pub type PROCPU_INT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn procpu_int(&self) -> PROCPU_INT_R {
        PROCPU_INT_R::new((self.bits & 0x03ff_ffff) as u32)
    }
}
