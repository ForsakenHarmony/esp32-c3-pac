#[doc = "Reader of register RD_KEY4_DATA4"]
pub type R = crate::R<u32, super::RD_KEY4_DATA4>;
#[doc = "Reader of field `KEY4_DATA4`"]
pub type KEY4_DATA4_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key4_data4(&self) -> KEY4_DATA4_R {
        KEY4_DATA4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
