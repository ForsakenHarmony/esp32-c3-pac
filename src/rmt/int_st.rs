#[doc = "Reader of register INT_ST"]
pub type R = crate::R<u32, super::INT_ST>;
#[doc = "Reader of field `CH1_TX_LOOP_INT_ST`"]
pub type CH1_TX_LOOP_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH0_TX_LOOP_INT_ST`"]
pub type CH0_TX_LOOP_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH3_RX_THR_EVENT_INT_ST`"]
pub type CH3_RX_THR_EVENT_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH2_RX_THR_EVENT_INT_ST`"]
pub type CH2_RX_THR_EVENT_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH1_TX_THR_EVENT_INT_ST`"]
pub type CH1_TX_THR_EVENT_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH0_TX_THR_EVENT_INT_ST`"]
pub type CH0_TX_THR_EVENT_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH3_ERR_INT_ST`"]
pub type CH3_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH2_ERR_INT_ST`"]
pub type CH2_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH1_ERR_INT_ST`"]
pub type CH1_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH0_ERR_INT_ST`"]
pub type CH0_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH3_RX_END_INT_ST`"]
pub type CH3_RX_END_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH2_RX_END_INT_ST`"]
pub type CH2_RX_END_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH1_TX_END_INT_ST`"]
pub type CH1_TX_END_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH0_TX_END_INT_ST`"]
pub type CH0_TX_END_INT_ST_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ch1_tx_loop_int_st(&self) -> CH1_TX_LOOP_INT_ST_R {
        CH1_TX_LOOP_INT_ST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ch0_tx_loop_int_st(&self) -> CH0_TX_LOOP_INT_ST_R {
        CH0_TX_LOOP_INT_ST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ch3_rx_thr_event_int_st(&self) -> CH3_RX_THR_EVENT_INT_ST_R {
        CH3_RX_THR_EVENT_INT_ST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ch2_rx_thr_event_int_st(&self) -> CH2_RX_THR_EVENT_INT_ST_R {
        CH2_RX_THR_EVENT_INT_ST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ch1_tx_thr_event_int_st(&self) -> CH1_TX_THR_EVENT_INT_ST_R {
        CH1_TX_THR_EVENT_INT_ST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ch0_tx_thr_event_int_st(&self) -> CH0_TX_THR_EVENT_INT_ST_R {
        CH0_TX_THR_EVENT_INT_ST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ch3_err_int_st(&self) -> CH3_ERR_INT_ST_R {
        CH3_ERR_INT_ST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ch2_err_int_st(&self) -> CH2_ERR_INT_ST_R {
        CH2_ERR_INT_ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ch1_err_int_st(&self) -> CH1_ERR_INT_ST_R {
        CH1_ERR_INT_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ch0_err_int_st(&self) -> CH0_ERR_INT_ST_R {
        CH0_ERR_INT_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ch3_rx_end_int_st(&self) -> CH3_RX_END_INT_ST_R {
        CH3_RX_END_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ch2_rx_end_int_st(&self) -> CH2_RX_END_INT_ST_R {
        CH2_RX_END_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ch1_tx_end_int_st(&self) -> CH1_TX_END_INT_ST_R {
        CH1_TX_END_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ch0_tx_end_int_st(&self) -> CH0_TX_END_INT_ST_R {
        CH0_TX_END_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
