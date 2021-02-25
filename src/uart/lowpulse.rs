#[doc = "Reader of register LOWPULSE"]
pub type R = crate::R<u32, super::LOWPULSE>;
#[doc = "Reader of field `LOWPULSE_MIN_CNT`"]
pub type LOWPULSE_MIN_CNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn lowpulse_min_cnt(&self) -> LOWPULSE_MIN_CNT_R {
        LOWPULSE_MIN_CNT_R::new((self.bits & 0x0fff) as u16)
    }
}
