#[doc = "Reader of register RX_HEAD"]
pub type R = crate::R<u32, super::RX_HEAD>;
#[doc = "Reader of field `RX_HEAD`"]
pub type RX_HEAD_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rx_head(&self) -> RX_HEAD_R {
        RX_HEAD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
