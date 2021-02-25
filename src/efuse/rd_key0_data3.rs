#[doc = "Reader of register RD_KEY0_DATA3"]
pub type R = crate::R<u32, super::RD_KEY0_DATA3>;
#[doc = "Reader of field `KEY0_DATA3`"]
pub type KEY0_DATA3_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key0_data3(&self) -> KEY0_DATA3_R {
        KEY0_DATA3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
