#[doc = "Reader of register TIME_HIGH1"]
pub type R = crate::R<u32, super::TIME_HIGH1>;
#[doc = "Reader of field `TIMER_VALUE1_HIGH`"]
pub type TIMER_VALUE1_HIGH_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn timer_value1_high(&self) -> TIMER_VALUE1_HIGH_R {
        TIMER_VALUE1_HIGH_R::new((self.bits & 0xffff) as u16)
    }
}
