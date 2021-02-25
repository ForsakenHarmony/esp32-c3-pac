#[doc = "Reader of register NEGPULSE"]
pub type R = crate::R<u32, super::NEGPULSE>;
#[doc = "Reader of field `NEGEDGE_MIN_CNT`"]
pub type NEGEDGE_MIN_CNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn negedge_min_cnt(&self) -> NEGEDGE_MIN_CNT_R {
        NEGEDGE_MIN_CNT_R::new((self.bits & 0x0fff) as u16)
    }
}
