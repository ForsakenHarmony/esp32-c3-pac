#[doc = "Reader of register STATE0"]
pub type R = crate::R<u32, super::STATE0>;
#[doc = "Reader of field `DECODE_STATE`"]
pub type DECODE_STATE_R = crate::R<u8, u8>;
#[doc = "Reader of field `RX_ERR_CAUSE`"]
pub type RX_ERR_CAUSE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn decode_state(&self) -> DECODE_STATE_R {
        DECODE_STATE_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rx_err_cause(&self) -> RX_ERR_CAUSE_R {
        RX_ERR_CAUSE_R::new((self.bits & 0x07) as u8)
    }
}
