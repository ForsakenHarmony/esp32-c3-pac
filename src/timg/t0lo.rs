#[doc = "Reader of register T0LO"]
pub type R = crate::R<u32, super::T0LO>;
#[doc = "Reader of field `T0_LO`"]
pub type T0_LO_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn t0_lo(&self) -> T0_LO_R {
        T0_LO_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
