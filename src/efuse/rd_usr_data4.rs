#[doc = "Reader of register RD_USR_DATA4"]
pub type R = crate::R<u32, super::RD_USR_DATA4>;
#[doc = "Reader of field `USR_DATA4`"]
pub type USR_DATA4_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn usr_data4(&self) -> USR_DATA4_R {
        USR_DATA4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
