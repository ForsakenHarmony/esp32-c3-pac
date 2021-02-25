#[doc = "Reader of register RD_KEY3_DATA4"]
pub type R = crate::R<u32, super::RD_KEY3_DATA4>;
#[doc = "Reader of field `KEY3_DATA4`"]
pub type KEY3_DATA4_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key3_data4(&self) -> KEY3_DATA4_R {
        KEY3_DATA4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
