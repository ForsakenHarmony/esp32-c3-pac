#[doc = "Reader of register RD_REPEAT_DATA4"]
pub type R = crate::R<u32, super::RD_REPEAT_DATA4>;
#[doc = "Reader of field `RPT4_RESERVED4`"]
pub type RPT4_RESERVED4_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn rpt4_reserved4(&self) -> RPT4_RESERVED4_R {
        RPT4_RESERVED4_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
