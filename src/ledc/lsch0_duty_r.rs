#[doc = "Reader of register LSCH0_DUTY_R"]
pub type R = crate::R<u32, super::LSCH0_DUTY_R>;
#[doc = "Reader of field `DUTY_LSCH0`"]
pub type DUTY_LSCH0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn duty_lsch0(&self) -> DUTY_LSCH0_R {
        DUTY_LSCH0_R::new((self.bits & 0x0007_ffff) as u32)
    }
}
