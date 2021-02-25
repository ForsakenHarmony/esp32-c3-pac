#[doc = "Reader of register RD_USR_DATA1"]
pub type R = crate::R<u32, super::RD_USR_DATA1>;
#[doc = "Reader of field `USR_DATA1`"]
pub type USR_DATA1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn usr_data1(&self) -> USR_DATA1_R {
        USR_DATA1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
