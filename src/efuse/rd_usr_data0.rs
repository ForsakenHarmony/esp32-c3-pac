#[doc = "Reader of register RD_USR_DATA0"]
pub type R = crate::R<u32, super::RD_USR_DATA0>;
#[doc = "Reader of field `USR_DATA0`"]
pub type USR_DATA0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn usr_data0(&self) -> USR_DATA0_R {
        USR_DATA0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
