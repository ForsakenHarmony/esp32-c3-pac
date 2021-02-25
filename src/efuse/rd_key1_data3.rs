#[doc = "Reader of register RD_KEY1_DATA3"]
pub type R = crate::R<u32, super::RD_KEY1_DATA3>;
#[doc = "Reader of field `KEY1_DATA3`"]
pub type KEY1_DATA3_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key1_data3(&self) -> KEY1_DATA3_R {
        KEY1_DATA3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
