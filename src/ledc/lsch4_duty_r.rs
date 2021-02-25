#[doc = "Reader of register LSCH4_DUTY_R"]
pub type R = crate::R<u32, super::LSCH4_DUTY_R>;
#[doc = "Reader of field `DUTY_LSCH4`"]
pub type DUTY_LSCH4_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn duty_lsch4(&self) -> DUTY_LSCH4_R {
        DUTY_LSCH4_R::new((self.bits & 0x0007_ffff) as u32)
    }
}
