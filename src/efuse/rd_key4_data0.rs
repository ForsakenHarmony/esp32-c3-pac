#[doc = "Reader of register RD_KEY4_DATA0"]
pub type R = crate::R<u32, super::RD_KEY4_DATA0>;
#[doc = "Reader of field `KEY4_DATA0`"]
pub type KEY4_DATA0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key4_data0(&self) -> KEY4_DATA0_R {
        KEY4_DATA0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
