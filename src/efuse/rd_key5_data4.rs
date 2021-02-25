#[doc = "Reader of register RD_KEY5_DATA4"]
pub type R = crate::R<u32, super::RD_KEY5_DATA4>;
#[doc = "Reader of field `KEY5_DATA4`"]
pub type KEY5_DATA4_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key5_data4(&self) -> KEY5_DATA4_R {
        KEY5_DATA4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
