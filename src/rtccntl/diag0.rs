#[doc = "Reader of register DIAG0"]
pub type R = crate::R<u32, super::DIAG0>;
#[doc = "Reader of field `LOW_POWER_DIAG1`"]
pub type LOW_POWER_DIAG1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn low_power_diag1(&self) -> LOW_POWER_DIAG1_R {
        LOW_POWER_DIAG1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
