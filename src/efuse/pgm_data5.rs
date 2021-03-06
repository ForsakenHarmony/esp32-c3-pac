#[doc = "Reader of register PGM_DATA5"]
pub type R = crate::R<u32, super::PGM_DATA5>;
#[doc = "Reader of field `RPT4_RESERVED4`"]
pub type RPT4_RESERVED4_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn rpt4_reserved4(&self) -> RPT4_RESERVED4_R {
        RPT4_RESERVED4_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
