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
#[doc = "Reader of field `APP_CTRL1_INT_ENA`"]
pub type APP_CTRL1_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APP_CTRL1_INT_ENA`"]
pub struct APP_CTRL1_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CTRL1_INT_ENA_W<'a> {
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
#[doc = "Reader of field `APP_CTRL0_INT_ENA`"]
pub type APP_CTRL0_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APP_CTRL0_INT_ENA`"]
pub struct APP_CTRL0_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CTRL0_INT_ENA_W<'a> {
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
#[doc = "Reader of field `OUTLINK_EOF_ERR_INT_ENA`"]
pub type OUTLINK_EOF_ERR_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTLINK_EOF_ERR_INT_ENA`"]
pub struct OUTLINK_EOF_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTLINK_EOF_ERR_INT_ENA_W<'a> {
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
#[doc = "Reader of field `SEND_A_Q_INT_ENA`"]
pub type SEND_A_Q_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEND_A_Q_INT_ENA`"]
pub struct SEND_A_Q_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SEND_A_Q_INT_ENA_W<'a> {
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
#[doc = "Reader of field `SEND_S_Q_INT_ENA`"]
pub type SEND_S_Q_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEND_S_Q_INT_ENA`"]
pub struct SEND_S_Q_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SEND_S_Q_INT_ENA_W<'a> {
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
#[doc = "Reader of field `TX_HUNG_INT_ENA`"]
pub type TX_HUNG_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_HUNG_INT_ENA`"]
pub struct TX_HUNG_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_HUNG_INT_ENA_W<'a> {
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
#[doc = "Reader of field `RX_HUNG_INT_ENA`"]
pub type RX_HUNG_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_HUNG_INT_ENA`"]
pub struct RX_HUNG_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_HUNG_INT_ENA_W<'a> {
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
#[doc = "Reader of field `TX_START_INT_ENA`"]
pub type TX_START_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_START_INT_ENA`"]
pub struct TX_START_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_START_INT_ENA_W<'a> {
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
#[doc = "Reader of field `RX_START_INT_ENA`"]
pub type RX_START_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_START_INT_ENA`"]
pub struct RX_START_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_START_INT_ENA_W<'a> {
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
    pub fn app_ctrl1_int_ena(&self) -> APP_CTRL1_INT_ENA_R {
        APP_CTRL1_INT_ENA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn app_ctrl0_int_ena(&self) -> APP_CTRL0_INT_ENA_R {
        APP_CTRL0_INT_ENA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn outlink_eof_err_int_ena(&self) -> OUTLINK_EOF_ERR_INT_ENA_R {
        OUTLINK_EOF_ERR_INT_ENA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn send_a_q_int_ena(&self) -> SEND_A_Q_INT_ENA_R {
        SEND_A_Q_INT_ENA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn send_s_q_int_ena(&self) -> SEND_S_Q_INT_ENA_R {
        SEND_S_Q_INT_ENA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tx_hung_int_ena(&self) -> TX_HUNG_INT_ENA_R {
        TX_HUNG_INT_ENA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_hung_int_ena(&self) -> RX_HUNG_INT_ENA_R {
        RX_HUNG_INT_ENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_start_int_ena(&self) -> TX_START_INT_ENA_R {
        TX_START_INT_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_start_int_ena(&self) -> RX_START_INT_ENA_R {
        RX_START_INT_ENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn app_ctrl1_int_ena(&mut self) -> APP_CTRL1_INT_ENA_W {
        APP_CTRL1_INT_ENA_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn app_ctrl0_int_ena(&mut self) -> APP_CTRL0_INT_ENA_W {
        APP_CTRL0_INT_ENA_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn outlink_eof_err_int_ena(&mut self) -> OUTLINK_EOF_ERR_INT_ENA_W {
        OUTLINK_EOF_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn send_a_q_int_ena(&mut self) -> SEND_A_Q_INT_ENA_W {
        SEND_A_Q_INT_ENA_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn send_s_q_int_ena(&mut self) -> SEND_S_Q_INT_ENA_W {
        SEND_S_Q_INT_ENA_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tx_hung_int_ena(&mut self) -> TX_HUNG_INT_ENA_W {
        TX_HUNG_INT_ENA_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_hung_int_ena(&mut self) -> RX_HUNG_INT_ENA_W {
        RX_HUNG_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_start_int_ena(&mut self) -> TX_START_INT_ENA_W {
        TX_START_INT_ENA_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_start_int_ena(&mut self) -> RX_START_INT_ENA_W {
        RX_START_INT_ENA_W { w: self }
    }
}
