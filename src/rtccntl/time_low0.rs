#[doc = "Reader of register TIME_LOW0"]
pub type R = crate::R<u32, super::TIME_LOW0>;
#[doc = "Reader of field `TIMER_VALUE0_LOW`"]
pub type TIMER_VALUE0_LOW_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn timer_value0_low(&self) -> TIMER_VALUE0_LOW_R {
        TIMER_VALUE0_LOW_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
