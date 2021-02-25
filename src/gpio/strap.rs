#[doc = "Reader of register STRAP"]
pub type R = crate::R<u32, super::STRAP>;
#[doc = "Reader of field `STRAPPING`"]
pub type STRAPPING_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn strapping(&self) -> STRAPPING_R {
        STRAPPING_R::new((self.bits & 0xffff) as u16)
    }
}
