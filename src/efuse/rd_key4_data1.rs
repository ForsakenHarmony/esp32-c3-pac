#[doc = "Reader of register RD_KEY4_DATA1"]
pub type R = crate::R<u32, super::RD_KEY4_DATA1>;
#[doc = "Reader of field `KEY4_DATA1`"]
pub type KEY4_DATA1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key4_data1(&self) -> KEY4_DATA1_R {
        KEY4_DATA1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
