#[doc = "Reader of register RD_USR_DATA6"]
pub type R = crate::R<u32, super::RD_USR_DATA6>;
#[doc = "Reader of field `USR_DATA6`"]
pub type USR_DATA6_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn usr_data6(&self) -> USR_DATA6_R {
        USR_DATA6_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
