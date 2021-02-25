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
#[doc = "Reader of field `DETECT_START_INT_ENA`"]
pub type DETECT_START_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DETECT_START_INT_ENA`"]
pub struct DETECT_START_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DETECT_START_INT_ENA_W<'a> {
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
#[doc = "Reader of field `TX_DATA_INT_ENA`"]
pub type TX_DATA_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_DATA_INT_ENA`"]
pub struct TX_DATA_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DATA_INT_ENA_W<'a> {
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
#[doc = "Reader of field `RX_DATA_INT_ENA`"]
pub type RX_DATA_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_DATA_INT_ENA`"]
pub struct RX_DATA_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DATA_INT_ENA_W<'a> {
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
#[doc = "Reader of field `ACK_ERR_INT_ENA`"]
pub type ACK_ERR_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACK_ERR_INT_ENA`"]
pub struct ACK_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ACK_ERR_INT_ENA_W<'a> {
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
#[doc = "Reader of field `TIMEOUT_INT_ENA`"]
pub type TIMEOUT_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMEOUT_INT_ENA`"]
pub struct TIMEOUT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `MASTER_TRAN_COMP_INT_ENA`"]
pub type MASTER_TRAN_COMP_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASTER_TRAN_COMP_INT_ENA`"]
pub struct MASTER_TRAN_COMP_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_TRAN_COMP_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `SLAVE_TRAN_COMP_INT_ENA`"]
pub type SLAVE_TRAN_COMP_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLAVE_TRAN_COMP_INT_ENA`"]
pub struct SLAVE_TRAN_COMP_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_TRAN_COMP_INT_ENA_W<'a> {
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
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn detect_start_int_ena(&self) -> DETECT_START_INT_ENA_R {
        DETECT_START_INT_ENA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tx_data_int_ena(&self) -> TX_DATA_INT_ENA_R {
        TX_DATA_INT_ENA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rx_data_int_ena(&self) -> RX_DATA_INT_ENA_R {
        RX_DATA_INT_ENA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ack_err_int_ena(&self) -> ACK_ERR_INT_ENA_R {
        ACK_ERR_INT_ENA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn timeout_int_ena(&self) -> TIMEOUT_INT_ENA_R {
        TIMEOUT_INT_ENA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn trans_complete_int_ena(&self) -> TRANS_COMPLETE_INT_ENA_R {
        TRANS_COMPLETE_INT_ENA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn master_tran_comp_int_ena(&self) -> MASTER_TRAN_COMP_INT_ENA_R {
        MASTER_TRAN_COMP_INT_ENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn arbitration_lost_int_ena(&self) -> ARBITRATION_LOST_INT_ENA_R {
        ARBITRATION_LOST_INT_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slave_tran_comp_int_ena(&self) -> SLAVE_TRAN_COMP_INT_ENA_R {
        SLAVE_TRAN_COMP_INT_ENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn detect_start_int_ena(&mut self) -> DETECT_START_INT_ENA_W {
        DETECT_START_INT_ENA_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tx_data_int_ena(&mut self) -> TX_DATA_INT_ENA_W {
        TX_DATA_INT_ENA_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rx_data_int_ena(&mut self) -> RX_DATA_INT_ENA_W {
        RX_DATA_INT_ENA_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ack_err_int_ena(&mut self) -> ACK_ERR_INT_ENA_W {
        ACK_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn timeout_int_ena(&mut self) -> TIMEOUT_INT_ENA_W {
        TIMEOUT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn trans_complete_int_ena(&mut self) -> TRANS_COMPLETE_INT_ENA_W {
        TRANS_COMPLETE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn master_tran_comp_int_ena(&mut self) -> MASTER_TRAN_COMP_INT_ENA_W {
        MASTER_TRAN_COMP_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn arbitration_lost_int_ena(&mut self) -> ARBITRATION_LOST_INT_ENA_W {
        ARBITRATION_LOST_INT_ENA_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slave_tran_comp_int_ena(&mut self) -> SLAVE_TRAN_COMP_INT_ENA_W {
        SLAVE_TRAN_COMP_INT_ENA_W { w: self }
    }
}
