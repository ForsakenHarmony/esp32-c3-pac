#[doc = "Reader of register LSTIMER1_VALUE"]
pub type R = crate::R<u32, super::LSTIMER1_VALUE>;
#[doc = "Reader of field `LSTIMER1_CNT`"]
pub type LSTIMER1_CNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn lstimer1_cnt(&self) -> LSTIMER1_CNT_R {
        LSTIMER1_CNT_R::new((self.bits & 0x3fff) as u16)
    }
}
