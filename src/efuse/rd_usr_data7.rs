#[doc = "Reader of register RD_USR_DATA7"]
pub type R = crate::R<u32, super::RD_USR_DATA7>;
#[doc = "Reader of field `USR_DATA7`"]
pub type USR_DATA7_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn usr_data7(&self) -> USR_DATA7_R {
        USR_DATA7_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
