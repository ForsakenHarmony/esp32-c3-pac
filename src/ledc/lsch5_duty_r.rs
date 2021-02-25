#[doc = "Reader of register LSCH5_DUTY_R"]
pub type R = crate::R<u32, super::LSCH5_DUTY_R>;
#[doc = "Reader of field `DUTY_LSCH5`"]
pub type DUTY_LSCH5_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn duty_lsch5(&self) -> DUTY_LSCH5_R {
        DUTY_LSCH5_R::new((self.bits & 0x0007_ffff) as u32)
    }
}
