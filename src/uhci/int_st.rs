#[doc = "Reader of register INT_ST"]
pub type R = crate::R<u32, super::INT_ST>;
#[doc = "Reader of field `APP_CTRL1_INT_ST`"]
pub type APP_CTRL1_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `APP_CTRL0_INT_ST`"]
pub type APP_CTRL0_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `OUTLINK_EOF_ERR_INT_ST`"]
pub type OUTLINK_EOF_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SEND_A_Q_INT_ST`"]
pub type SEND_A_Q_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SEND_S_Q_INT_ST`"]
pub type SEND_S_Q_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `TX_HUNG_INT_ST`"]
pub type TX_HUNG_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX_HUNG_INT_ST`"]
pub type RX_HUNG_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `TX_START_INT_ST`"]
pub type TX_START_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX_START_INT_ST`"]
pub type RX_START_INT_ST_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn app_ctrl1_int_st(&self) -> APP_CTRL1_INT_ST_R {
        APP_CTRL1_INT_ST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn app_ctrl0_int_st(&self) -> APP_CTRL0_INT_ST_R {
        APP_CTRL0_INT_ST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn outlink_eof_err_int_st(&self) -> OUTLINK_EOF_ERR_INT_ST_R {
        OUTLINK_EOF_ERR_INT_ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn send_a_q_int_st(&self) -> SEND_A_Q_INT_ST_R {
        SEND_A_Q_INT_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn send_s_q_int_st(&self) -> SEND_S_Q_INT_ST_R {
        SEND_S_Q_INT_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
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
    pub fn tx_start_int_st(&self) -> TX_START_INT_ST_R {
        TX_START_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_start_int_st(&self) -> RX_START_INT_ST_R {
        RX_START_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
