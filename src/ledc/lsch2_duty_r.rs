#[doc = "Reader of register LSCH2_DUTY_R"]
pub type R = crate::R<u32, super::LSCH2_DUTY_R>;
#[doc = "Reader of field `DUTY_LSCH2`"]
pub type DUTY_LSCH2_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn duty_lsch2(&self) -> DUTY_LSCH2_R {
        DUTY_LSCH2_R::new((self.bits & 0x0007_ffff) as u32)
    }
}
