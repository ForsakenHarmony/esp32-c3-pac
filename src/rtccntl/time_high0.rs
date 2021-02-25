#[doc = "Reader of register TIME_HIGH0"]
pub type R = crate::R<u32, super::TIME_HIGH0>;
#[doc = "Reader of field `TIMER_VALUE0_HIGH`"]
pub type TIMER_VALUE0_HIGH_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn timer_value0_high(&self) -> TIMER_VALUE0_HIGH_R {
        TIMER_VALUE0_HIGH_R::new((self.bits & 0xffff) as u16)
    }
}
