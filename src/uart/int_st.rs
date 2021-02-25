#[doc = "Reader of register INT_ST"]
pub type R = crate::R<u32, super::INT_ST>;
#[doc = "Reader of field `WAKEUP_INT_ST`"]
pub type WAKEUP_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `AT_CMD_CHAR_DET_INT_ST`"]
pub type AT_CMD_CHAR_DET_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `RS485_CLASH_INT_ST`"]
pub type RS485_CLASH_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `RS485_FRM_ERR_INT_ST`"]
pub type RS485_FRM_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `RS485_PARITY_ERR_INT_ST`"]
pub type RS485_PARITY_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `TX_DONE_INT_ST`"]
pub type TX_DONE_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `TX_BRK_IDLE_DONE_INT_ST`"]
pub type TX_BRK_IDLE_DONE_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `TX_BRK_DONE_INT_ST`"]
pub type TX_BRK_DONE_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `GLITCH_DET_INT_ST`"]
pub type GLITCH_DET_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SW_XOFF_INT_ST`"]
pub type SW_XOFF_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SW_XON_INT_ST`"]
pub type SW_XON_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFIFO_TOUT_INT_ST`"]
pub type RXFIFO_TOUT_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `BRK_DET_INT_ST`"]
pub type BRK_DET_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTS_CHG_INT_ST`"]
pub type CTS_CHG_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `DSR_CHG_INT_ST`"]
pub type DSR_CHG_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFIFO_OVF_INT_ST`"]
pub type RXFIFO_OVF_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `FRM_ERR_INT_ST`"]
pub type FRM_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `PARITY_ERR_INT_ST`"]
pub type PARITY_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXFIFO_EMPTY_INT_ST`"]
pub type TXFIFO_EMPTY_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFIFO_FULL_INT_ST`"]
pub type RXFIFO_FULL_INT_ST_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn wakeup_int_st(&self) -> WAKEUP_INT_ST_R {
        WAKEUP_INT_ST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn at_cmd_char_det_int_st(&self) -> AT_CMD_CHAR_DET_INT_ST_R {
        AT_CMD_CHAR_DET_INT_ST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rs485_clash_int_st(&self) -> RS485_CLASH_INT_ST_R {
        RS485_CLASH_INT_ST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rs485_frm_err_int_st(&self) -> RS485_FRM_ERR_INT_ST_R {
        RS485_FRM_ERR_INT_ST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rs485_parity_err_int_st(&self) -> RS485_PARITY_ERR_INT_ST_R {
        RS485_PARITY_ERR_INT_ST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tx_done_int_st(&self) -> TX_DONE_INT_ST_R {
        TX_DONE_INT_ST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tx_brk_idle_done_int_st(&self) -> TX_BRK_IDLE_DONE_INT_ST_R {
        TX_BRK_IDLE_DONE_INT_ST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tx_brk_done_int_st(&self) -> TX_BRK_DONE_INT_ST_R {
        TX_BRK_DONE_INT_ST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn glitch_det_int_st(&self) -> GLITCH_DET_INT_ST_R {
        GLITCH_DET_INT_ST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sw_xoff_int_st(&self) -> SW_XOFF_INT_ST_R {
        SW_XOFF_INT_ST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sw_xon_int_st(&self) -> SW_XON_INT_ST_R {
        SW_XON_INT_ST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rxfifo_tout_int_st(&self) -> RXFIFO_TOUT_INT_ST_R {
        RXFIFO_TOUT_INT_ST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn brk_det_int_st(&self) -> BRK_DET_INT_ST_R {
        BRK_DET_INT_ST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cts_chg_int_st(&self) -> CTS_CHG_INT_ST_R {
        CTS_CHG_INT_ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dsr_chg_int_st(&self) -> DSR_CHG_INT_ST_R {
        DSR_CHG_INT_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rxfifo_ovf_int_st(&self) -> RXFIFO_OVF_INT_ST_R {
        RXFIFO_OVF_INT_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn frm_err_int_st(&self) -> FRM_ERR_INT_ST_R {
        FRM_ERR_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn parity_err_int_st(&self) -> PARITY_ERR_INT_ST_R {
        PARITY_ERR_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn txfifo_empty_int_st(&self) -> TXFIFO_EMPTY_INT_ST_R {
        TXFIFO_EMPTY_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rxfifo_full_int_st(&self) -> RXFIFO_FULL_INT_ST_R {
        RXFIFO_FULL_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
