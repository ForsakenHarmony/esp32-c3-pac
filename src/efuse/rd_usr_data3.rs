#[doc = "Reader of register RD_USR_DATA3"]
pub type R = crate::R<u32, super::RD_USR_DATA3>;
#[doc = "Reader of field `USR_DATA3`"]
pub type USR_DATA3_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn usr_data3(&self) -> USR_DATA3_R {
        USR_DATA3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
