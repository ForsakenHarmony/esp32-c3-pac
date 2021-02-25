#[doc = "Reader of register HIGHPULSE"]
pub type R = crate::R<u32, super::HIGHPULSE>;
#[doc = "Reader of field `HIGHPULSE_MIN_CNT`"]
pub type HIGHPULSE_MIN_CNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn highpulse_min_cnt(&self) -> HIGHPULSE_MIN_CNT_R {
        HIGHPULSE_MIN_CNT_R::new((self.bits & 0x0fff) as u16)
    }
}
