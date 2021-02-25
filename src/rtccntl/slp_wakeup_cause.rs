#[doc = "Reader of register SLP_WAKEUP_CAUSE"]
pub type R = crate::R<u32, super::SLP_WAKEUP_CAUSE>;
#[doc = "Reader of field `WAKEUP_CAUSE`"]
pub type WAKEUP_CAUSE_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:16"]
    #[inline(always)]
    pub fn wakeup_cause(&self) -> WAKEUP_CAUSE_R {
        WAKEUP_CAUSE_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
