#[doc = "Reader of register INT_ENA"]
pub type R = crate::R<u32, super::INT_ENA>;
#[doc = "Writer for register INT_ENA"]
pub type W = crate::W<u32, super::INT_ENA>;
#[doc = "Register INT_ENA `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_ENA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GENERAL_CALL_INT_ENA`"]
pub type GENERAL_CALL_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GENERAL_CALL_INT_ENA`"]
pub struct GENERAL_CALL_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> GENERAL_CALL_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `SLAVE_STRETCH_INT_ENA`"]
pub type SLAVE_STRETCH_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLAVE_STRETCH_INT_ENA`"]
pub struct SLAVE_STRETCH_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_STRETCH_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `DET_START_INT_ENA`"]
pub type DET_START_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DET_START_INT_ENA`"]
pub struct DET_START_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DET_START_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `SCL_MAIN_ST_TO_INT_ENA`"]
pub type SCL_MAIN_ST_TO_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCL_MAIN_ST_TO_INT_ENA`"]
pub struct SCL_MAIN_ST_TO_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_MAIN_ST_TO_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `SCL_ST_TO_INT_ENA`"]
pub type SCL_ST_TO_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCL_ST_TO_INT_ENA`"]
pub struct SCL_ST_TO_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_ST_TO_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `RXFIFO_UDF_INT_ENA`"]
pub type RXFIFO_UDF_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFIFO_UDF_INT_ENA`"]
pub struct RXFIFO_UDF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_UDF_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `TXFIFO_OVF_INT_ENA`"]
pub type TXFIFO_OVF_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFIFO_OVF_INT_ENA`"]
pub struct TXFIFO_OVF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFO_OVF_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `NACK_INT_ENA`"]
pub type NACK_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NACK_INT_ENA`"]
pub struct NACK_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> NACK_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `TRANS_START_INT_ENA`"]
pub type TRANS_START_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRANS_START_INT_ENA`"]
pub struct TRANS_START_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANS_START_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `TIME_OUT_INT_ENA`"]
pub type TIME_OUT_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIME_OUT_INT_ENA`"]
pub struct TIME_OUT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TIME_OUT_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `TRANS_COMPLETE_INT_ENA`"]
pub type TRANS_COMPLETE_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRANS_COMPLETE_INT_ENA`"]
pub struct TRANS_COMPLETE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANS_COMPLETE_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `MST_TXFIFO_UDF_INT_ENA`"]
pub type MST_TXFIFO_UDF_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MST_TXFIFO_UDF_INT_ENA`"]
pub struct MST_TXFIFO_UDF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> MST_TXFIFO_UDF_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `ARBITRATION_LOST_INT_ENA`"]
pub type ARBITRATION_LOST_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARBITRATION_LOST_INT_ENA`"]
pub struct ARBITRATION_LOST_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBITRATION_LOST_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `BYTE_TRANS_DONE_INT_ENA`"]
pub type BYTE_TRANS_DONE_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BYTE_TRANS_DONE_INT_ENA`"]
pub struct BYTE_TRANS_DONE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE_TRANS_DONE_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `END_DETECT_INT_ENA`"]
pub type END_DETECT_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `END_DETECT_INT_ENA`"]
pub struct END_DETECT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> END_DETECT_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `RXFIFO_OVF_INT_ENA`"]
pub type RXFIFO_OVF_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFIFO_OVF_INT_ENA`"]
pub struct RXFIFO_OVF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_OVF_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `TXFIFO_WM_INT_ENA`"]
pub type TXFIFO_WM_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFIFO_WM_INT_ENA`"]
pub struct TXFIFO_WM_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFO_WM_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `RXFIFO_WM_INT_ENA`"]
pub type RXFIFO_WM_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFIFO_WM_INT_ENA`"]
pub struct RXFIFO_WM_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_WM_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn general_call_int_ena(&self) -> GENERAL_CALL_INT_ENA_R {
        GENERAL_CALL_INT_ENA_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slave_stretch_int_ena(&self) -> SLAVE_STRETCH_INT_ENA_R {
        SLAVE_STRETCH_INT_ENA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn det_start_int_ena(&self) -> DET_START_INT_ENA_R {
        DET_START_INT_ENA_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn scl_main_st_to_int_ena(&self) -> SCL_MAIN_ST_TO_INT_ENA_R {
        SCL_MAIN_ST_TO_INT_ENA_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn scl_st_to_int_ena(&self) -> SCL_ST_TO_INT_ENA_R {
        SCL_ST_TO_INT_ENA_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rxfifo_udf_int_ena(&self) -> RXFIFO_UDF_INT_ENA_R {
        RXFIFO_UDF_INT_ENA_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn txfifo_ovf_int_ena(&self) -> TXFIFO_OVF_INT_ENA_R {
        TXFIFO_OVF_INT_ENA_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn nack_int_ena(&self) -> NACK_INT_ENA_R {
        NACK_INT_ENA_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn trans_start_int_ena(&self) -> TRANS_START_INT_ENA_R {
        TRANS_START_INT_ENA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn time_out_int_ena(&self) -> TIME_OUT_INT_ENA_R {
        TIME_OUT_INT_ENA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn trans_complete_int_ena(&self) -> TRANS_COMPLETE_INT_ENA_R {
        TRANS_COMPLETE_INT_ENA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn mst_txfifo_udf_int_ena(&self) -> MST_TXFIFO_UDF_INT_ENA_R {
        MST_TXFIFO_UDF_INT_ENA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn arbitration_lost_int_ena(&self) -> ARBITRATION_LOST_INT_ENA_R {
        ARBITRATION_LOST_INT_ENA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn byte_trans_done_int_ena(&self) -> BYTE_TRANS_DONE_INT_ENA_R {
        BYTE_TRANS_DONE_INT_ENA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn end_detect_int_ena(&self) -> END_DETECT_INT_ENA_R {
        END_DETECT_INT_ENA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rxfifo_ovf_int_ena(&self) -> RXFIFO_OVF_INT_ENA_R {
        RXFIFO_OVF_INT_ENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn txfifo_wm_int_ena(&self) -> TXFIFO_WM_INT_ENA_R {
        TXFIFO_WM_INT_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rxfifo_wm_int_ena(&self) -> RXFIFO_WM_INT_ENA_R {
        RXFIFO_WM_INT_ENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn general_call_int_ena(&mut self) -> GENERAL_CALL_INT_ENA_W {
        GENERAL_CALL_INT_ENA_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slave_stretch_int_ena(&mut self) -> SLAVE_STRETCH_INT_ENA_W {
        SLAVE_STRETCH_INT_ENA_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn det_start_int_ena(&mut self) -> DET_START_INT_ENA_W {
        DET_START_INT_ENA_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn scl_main_st_to_int_ena(&mut self) -> SCL_MAIN_ST_TO_INT_ENA_W {
        SCL_MAIN_ST_TO_INT_ENA_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn scl_st_to_int_ena(&mut self) -> SCL_ST_TO_INT_ENA_W {
        SCL_ST_TO_INT_ENA_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rxfifo_udf_int_ena(&mut self) -> RXFIFO_UDF_INT_ENA_W {
        RXFIFO_UDF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn txfifo_ovf_int_ena(&mut self) -> TXFIFO_OVF_INT_ENA_W {
        TXFIFO_OVF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn nack_int_ena(&mut self) -> NACK_INT_ENA_W {
        NACK_INT_ENA_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn trans_start_int_ena(&mut self) -> TRANS_START_INT_ENA_W {
        TRANS_START_INT_ENA_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn time_out_int_ena(&mut self) -> TIME_OUT_INT_ENA_W {
        TIME_OUT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn trans_complete_int_ena(&mut self) -> TRANS_COMPLETE_INT_ENA_W {
        TRANS_COMPLETE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn mst_txfifo_udf_int_ena(&mut self) -> MST_TXFIFO_UDF_INT_ENA_W {
        MST_TXFIFO_UDF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn arbitration_lost_int_ena(&mut self) -> ARBITRATION_LOST_INT_ENA_W {
        ARBITRATION_LOST_INT_ENA_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn byte_trans_done_int_ena(&mut self) -> BYTE_TRANS_DONE_INT_ENA_W {
        BYTE_TRANS_DONE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn end_detect_int_ena(&mut self) -> END_DETECT_INT_ENA_W {
        END_DETECT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rxfifo_ovf_int_ena(&mut self) -> RXFIFO_OVF_INT_ENA_W {
        RXFIFO_OVF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn txfifo_wm_int_ena(&mut self) -> TXFIFO_WM_INT_ENA_W {
        TXFIFO_WM_INT_ENA_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rxfifo_wm_int_ena(&mut self) -> RXFIFO_WM_INT_ENA_W {
        RXFIFO_WM_INT_ENA_W { w: self }
    }
}
