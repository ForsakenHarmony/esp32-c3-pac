#[doc = "Reader of register LSTIMER2_VALUE"]
pub type R = crate::R<u32, super::LSTIMER2_VALUE>;
#[doc = "Reader of field `LSTIMER2_CNT`"]
pub type LSTIMER2_CNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn lstimer2_cnt(&self) -> LSTIMER2_CNT_R {
        LSTIMER2_CNT_R::new((self.bits & 0x3fff) as u16)
    }
}
