#[doc = "Reader of register RD_KEY3_DATA7"]
pub type R = crate::R<u32, super::RD_KEY3_DATA7>;
#[doc = "Reader of field `KEY3_DATA7`"]
pub type KEY3_DATA7_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key3_data7(&self) -> KEY3_DATA7_R {
        KEY3_DATA7_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
