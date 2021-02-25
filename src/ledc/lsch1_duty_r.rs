#[doc = "Reader of register LSCH1_DUTY_R"]
pub type R = crate::R<u32, super::LSCH1_DUTY_R>;
#[doc = "Reader of field `DUTY_LSCH1`"]
pub type DUTY_LSCH1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn duty_lsch1(&self) -> DUTY_LSCH1_R {
        DUTY_LSCH1_R::new((self.bits & 0x0007_ffff) as u32)
    }
}
