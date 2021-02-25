#[doc = "Reader of register TIME_LOW1"]
pub type R = crate::R<u32, super::TIME_LOW1>;
#[doc = "Reader of field `TIMER_VALUE1_LOW`"]
pub type TIMER_VALUE1_LOW_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn timer_value1_low(&self) -> TIMER_VALUE1_LOW_R {
        TIMER_VALUE1_LOW_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
