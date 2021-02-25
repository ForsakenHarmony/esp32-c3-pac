#[doc = "Reader of register INT_STATUS"]
pub type R = crate::R<u32, super::INT_STATUS>;
#[doc = "Reader of field `GENERAL_CALL_INT_ST`"]
pub type GENERAL_CALL_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SLAVE_STRETCH_INT_ST`"]
pub type SLAVE_STRETCH_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `DET_START_INT_ST`"]
pub type DET_START_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SCL_MAIN_ST_TO_INT_ST`"]
pub type SCL_MAIN_ST_TO_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SCL_ST_TO_INT_ST`"]
pub type SCL_ST_TO_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFIFO_UDF_INT_ST`"]
pub type RXFIFO_UDF_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXFIFO_OVF_INT_ST`"]
pub type TXFIFO_OVF_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `NACK_INT_ST`"]
pub type NACK_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRANS_START_INT_ST`"]
pub type TRANS_START_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIME_OUT_INT_ST`"]
pub type TIME_OUT_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRANS_COMPLETE_INT_ST`"]
pub type TRANS_COMPLETE_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `MST_TXFIFO_UDF_INT_ST`"]
pub type MST_TXFIFO_UDF_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `ARBITRATION_LOST_INT_ST`"]
pub type ARBITRATION_LOST_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `BYTE_TRANS_DONE_INT_ST`"]
pub type BYTE_TRANS_DONE_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `END_DETECT_INT_ST`"]
pub type END_DETECT_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFIFO_OVF_INT_ST`"]
pub type RXFIFO_OVF_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXFIFO_WM_INT_ST`"]
pub type TXFIFO_WM_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFIFO_WM_INT_ST`"]
pub type RXFIFO_WM_INT_ST_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn general_call_int_st(&self) -> GENERAL_CALL_INT_ST_R {
        GENERAL_CALL_INT_ST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slave_stretch_int_st(&self) -> SLAVE_STRETCH_INT_ST_R {
        SLAVE_STRETCH_INT_ST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn det_start_int_st(&self) -> DET_START_INT_ST_R {
        DET_START_INT_ST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn scl_main_st_to_int_st(&self) -> SCL_MAIN_ST_TO_INT_ST_R {
        SCL_MAIN_ST_TO_INT_ST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn scl_st_to_int_st(&self) -> SCL_ST_TO_INT_ST_R {
        SCL_ST_TO_INT_ST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rxfifo_udf_int_st(&self) -> RXFIFO_UDF_INT_ST_R {
        RXFIFO_UDF_INT_ST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn txfifo_ovf_int_st(&self) -> TXFIFO_OVF_INT_ST_R {
        TXFIFO_OVF_INT_ST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn nack_int_st(&self) -> NACK_INT_ST_R {
        NACK_INT_ST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn trans_start_int_st(&self) -> TRANS_START_INT_ST_R {
        TRANS_START_INT_ST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn time_out_int_st(&self) -> TIME_OUT_INT_ST_R {
        TIME_OUT_INT_ST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn trans_complete_int_st(&self) -> TRANS_COMPLETE_INT_ST_R {
        TRANS_COMPLETE_INT_ST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn mst_txfifo_udf_int_st(&self) -> MST_TXFIFO_UDF_INT_ST_R {
        MST_TXFIFO_UDF_INT_ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn arbitration_lost_int_st(&self) -> ARBITRATION_LOST_INT_ST_R {
        ARBITRATION_LOST_INT_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn byte_trans_done_int_st(&self) -> BYTE_TRANS_DONE_INT_ST_R {
        BYTE_TRANS_DONE_INT_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn end_detect_int_st(&self) -> END_DETECT_INT_ST_R {
        END_DETECT_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rxfifo_ovf_int_st(&self) -> RXFIFO_OVF_INT_ST_R {
        RXFIFO_OVF_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn txfifo_wm_int_st(&self) -> TXFIFO_WM_INT_ST_R {
        TXFIFO_WM_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rxfifo_wm_int_st(&self) -> RXFIFO_WM_INT_ST_R {
        RXFIFO_WM_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
