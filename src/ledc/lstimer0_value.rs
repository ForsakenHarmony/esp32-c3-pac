#[doc = "Reader of register LSTIMER0_VALUE"]
pub type R = crate::R<u32, super::LSTIMER0_VALUE>;
#[doc = "Reader of field `LSTIMER0_CNT`"]
pub type LSTIMER0_CNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn lstimer0_cnt(&self) -> LSTIMER0_CNT_R {
        LSTIMER0_CNT_R::new((self.bits & 0x3fff) as u16)
    }
}
