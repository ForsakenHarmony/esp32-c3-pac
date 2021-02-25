#[doc = "Reader of register T0HI"]
pub type R = crate::R<u32, super::T0HI>;
#[doc = "Reader of field `T0_HI`"]
pub type T0_HI_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:21"]
    #[inline(always)]
    pub fn t0_hi(&self) -> T0_HI_R {
        T0_HI_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
