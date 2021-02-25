#[doc = "Reader of register LSTIMER3_VALUE"]
pub type R = crate::R<u32, super::LSTIMER3_VALUE>;
#[doc = "Reader of field `LSTIMER3_CNT`"]
pub type LSTIMER3_CNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn lstimer3_cnt(&self) -> LSTIMER3_CNT_R {
        LSTIMER3_CNT_R::new((self.bits & 0x3fff) as u16)
    }
}
