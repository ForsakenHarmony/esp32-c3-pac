#[doc = "Reader of register LSCH3_DUTY_R"]
pub type R = crate::R<u32, super::LSCH3_DUTY_R>;
#[doc = "Reader of field `DUTY_LSCH3`"]
pub type DUTY_LSCH3_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn duty_lsch3(&self) -> DUTY_LSCH3_R {
        DUTY_LSCH3_R::new((self.bits & 0x0007_ffff) as u32)
    }
}
