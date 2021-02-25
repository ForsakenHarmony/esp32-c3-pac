#[doc = "Reader of register RD_KEY1_DATA6"]
pub type R = crate::R<u32, super::RD_KEY1_DATA6>;
#[doc = "Reader of field `KEY1_DATA6`"]
pub type KEY1_DATA6_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key1_data6(&self) -> KEY1_DATA6_R {
        KEY1_DATA6_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
