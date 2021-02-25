#[doc = "Reader of register INT_ST"]
pub type R = crate::R<u32, super::INT_ST>;
#[doc = "Reader of field `DETECT_START_INT_ST`"]
pub type DETECT_START_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `TX_DATA_INT_ST`"]
pub type TX_DATA_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX_DATA_INT_ST`"]
pub type RX_DATA_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACK_ERR_INT_ST`"]
pub type ACK_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIMEOUT_INT_ST`"]
pub type TIMEOUT_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRANS_COMPLETE_INT_ST`"]
pub type TRANS_COMPLETE_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `MASTER_TRAN_COMP_INT_ST`"]
pub type MASTER_TRAN_COMP_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `ARBITRATION_LOST_INT_ST`"]
pub type ARBITRATION_LOST_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SLAVE_TRAN_COMP_INT_ST`"]
pub type SLAVE_TRAN_COMP_INT_ST_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn detect_start_int_st(&self) -> DETECT_START_INT_ST_R {
        DETECT_START_INT_ST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tx_data_int_st(&self) -> TX_DATA_INT_ST_R {
        TX_DATA_INT_ST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rx_data_int_st(&self) -> RX_DATA_INT_ST_R {
        RX_DATA_INT_ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ack_err_int_st(&self) -> ACK_ERR_INT_ST_R {
        ACK_ERR_INT_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn timeout_int_st(&self) -> TIMEOUT_INT_ST_R {
        TIMEOUT_INT_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn trans_complete_int_st(&self) -> TRANS_COMPLETE_INT_ST_R {
        TRANS_COMPLETE_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn master_tran_comp_int_st(&self) -> MASTER_TRAN_COMP_INT_ST_R {
        MASTER_TRAN_COMP_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn arbitration_lost_int_st(&self) -> ARBITRATION_LOST_INT_ST_R {
        ARBITRATION_LOST_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slave_tran_comp_int_st(&self) -> SLAVE_TRAN_COMP_INT_ST_R {
        SLAVE_TRAN_COMP_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
