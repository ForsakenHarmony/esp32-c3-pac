#[doc = "Reader of register SLP_REJECT_CAUSE"]
pub type R = crate::R<u32, super::SLP_REJECT_CAUSE>;
#[doc = "Reader of field `REJECT_CAUSE`"]
pub type REJECT_CAUSE_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:17"]
    #[inline(always)]
    pub fn reject_cause(&self) -> REJECT_CAUSE_R {
        REJECT_CAUSE_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
