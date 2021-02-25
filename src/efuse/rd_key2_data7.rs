#[doc = "Reader of register RD_KEY2_DATA7"]
pub type R = crate::R<u32, super::RD_KEY2_DATA7>;
#[doc = "Reader of field `KEY2_DATA7`"]
pub type KEY2_DATA7_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key2_data7(&self) -> KEY2_DATA7_R {
        KEY2_DATA7_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
