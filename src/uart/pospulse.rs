#[doc = "Reader of register POSPULSE"]
pub type R = crate::R<u32, super::POSPULSE>;
#[doc = "Reader of field `POSEDGE_MIN_CNT`"]
pub type POSEDGE_MIN_CNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn posedge_min_cnt(&self) -> POSEDGE_MIN_CNT_R {
        POSEDGE_MIN_CNT_R::new((self.bits & 0x0fff) as u16)
    }
}
