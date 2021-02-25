#[doc = "Reader of register STATUS_NEXT"]
pub type R = crate::R<u32, super::STATUS_NEXT>;
#[doc = "Reader of field `STATUS_INTERRUPT_NEXT`"]
pub type STATUS_INTERRUPT_NEXT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn status_interrupt_next(&self) -> STATUS_INTERRUPT_NEXT_R {
        STATUS_INTERRUPT_NEXT_R::new((self.bits & 0x03ff_ffff) as u32)
    }
}
