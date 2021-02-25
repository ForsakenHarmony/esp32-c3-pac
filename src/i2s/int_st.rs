#[doc = "Reader of register INT_ST"]
pub type R = crate::R<u32, super::INT_ST>;
#[doc = "Reader of field `TX_HUNG_INT_ST`"]
pub type TX_HUNG_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX_HUNG_INT_ST`"]
pub type RX_HUNG_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `TX_DONE_INT_ST`"]
pub type TX_DONE_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX_DONE_INT_ST`"]
pub type RX_DONE_INT_ST_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tx_hung_int_st(&self) -> TX_HUNG_INT_ST_R {
        TX_HUNG_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_hung_int_st(&self) -> RX_HUNG_INT_ST_R {
        RX_HUNG_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_done_int_st(&self) -> TX_DONE_INT_ST_R {
        TX_DONE_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_done_int_st(&self) -> RX_DONE_INT_ST_R {
        RX_DONE_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
