#[doc = "Reader of register RD_USR_DATA5"]
pub type R = crate::R<u32, super::RD_USR_DATA5>;
#[doc = "Reader of field `USR_DATA5`"]
pub type USR_DATA5_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn usr_data5(&self) -> USR_DATA5_R {
        USR_DATA5_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
