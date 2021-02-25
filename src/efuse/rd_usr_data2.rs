#[doc = "Reader of register RD_USR_DATA2"]
pub type R = crate::R<u32, super::RD_USR_DATA2>;
#[doc = "Reader of field `USR_DATA2`"]
pub type USR_DATA2_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn usr_data2(&self) -> USR_DATA2_R {
        USR_DATA2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
