#[doc = "Writer for register INT_CLR"]
pub type W = crate::W<u32, super::INT_CLR>;
#[doc = "Register INT_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `WAKEUP_INT_CLR`"]
pub struct WAKEUP_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Write proxy for field `AT_CMD_CHAR_DET_INT_CLR`"]
pub struct AT_CMD_CHAR_DET_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> AT_CMD_CHAR_DET_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Write proxy for field `RS485_CLASH_INT_CLR`"]
pub struct RS485_CLASH_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RS485_CLASH_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `RS485_FRM_ERR_INT_CLR`"]
pub struct RS485_FRM_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RS485_FRM_ERR_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `RS485_PARITY_ERR_INT_CLR`"]
pub struct RS485_PARITY_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RS485_PARITY_ERR_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `TX_DONE_INT_CLR`"]
pub struct TX_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DONE_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `TX_BRK_IDLE_DONE_INT_CLR`"]
pub struct TX_BRK_IDLE_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BRK_IDLE_DONE_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `TX_BRK_DONE_INT_CLR`"]
pub struct TX_BRK_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BRK_DONE_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `GLITCH_DET_INT_CLR`"]
pub struct GLITCH_DET_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GLITCH_DET_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `SW_XOFF_INT_CLR`"]
pub struct SW_XOFF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_XOFF_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `SW_XON_INT_CLR`"]
pub struct SW_XON_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_XON_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `RXFIFO_TOUT_INT_CLR`"]
pub struct RXFIFO_TOUT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_TOUT_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `BRK_DET_INT_CLR`"]
pub struct BRK_DET_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> BRK_DET_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `CTS_CHG_INT_CLR`"]
pub struct CTS_CHG_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CTS_CHG_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `DSR_CHG_INT_CLR`"]
pub struct DSR_CHG_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DSR_CHG_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `RXFIFO_OVF_INT_CLR`"]
pub struct RXFIFO_OVF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_OVF_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `FRM_ERR_INT_CLR`"]
pub struct FRM_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> FRM_ERR_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `PARITY_ERR_INT_CLR`"]
pub struct PARITY_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITY_ERR_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `TXFIFO_EMPTY_INT_CLR`"]
pub struct TXFIFO_EMPTY_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFO_EMPTY_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `RXFIFO_FULL_INT_CLR`"]
pub struct RXFIFO_FULL_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_FULL_INT_CLR_W<'a> {
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
impl W {
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn wakeup_int_clr(&mut self) -> WAKEUP_INT_CLR_W {
        WAKEUP_INT_CLR_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn at_cmd_char_det_int_clr(&mut self) -> AT_CMD_CHAR_DET_INT_CLR_W {
        AT_CMD_CHAR_DET_INT_CLR_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rs485_clash_int_clr(&mut self) -> RS485_CLASH_INT_CLR_W {
        RS485_CLASH_INT_CLR_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rs485_frm_err_int_clr(&mut self) -> RS485_FRM_ERR_INT_CLR_W {
        RS485_FRM_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rs485_parity_err_int_clr(&mut self) -> RS485_PARITY_ERR_INT_CLR_W {
        RS485_PARITY_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tx_done_int_clr(&mut self) -> TX_DONE_INT_CLR_W {
        TX_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tx_brk_idle_done_int_clr(&mut self) -> TX_BRK_IDLE_DONE_INT_CLR_W {
        TX_BRK_IDLE_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tx_brk_done_int_clr(&mut self) -> TX_BRK_DONE_INT_CLR_W {
        TX_BRK_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn glitch_det_int_clr(&mut self) -> GLITCH_DET_INT_CLR_W {
        GLITCH_DET_INT_CLR_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sw_xoff_int_clr(&mut self) -> SW_XOFF_INT_CLR_W {
        SW_XOFF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sw_xon_int_clr(&mut self) -> SW_XON_INT_CLR_W {
        SW_XON_INT_CLR_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rxfifo_tout_int_clr(&mut self) -> RXFIFO_TOUT_INT_CLR_W {
        RXFIFO_TOUT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn brk_det_int_clr(&mut self) -> BRK_DET_INT_CLR_W {
        BRK_DET_INT_CLR_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cts_chg_int_clr(&mut self) -> CTS_CHG_INT_CLR_W {
        CTS_CHG_INT_CLR_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dsr_chg_int_clr(&mut self) -> DSR_CHG_INT_CLR_W {
        DSR_CHG_INT_CLR_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rxfifo_ovf_int_clr(&mut self) -> RXFIFO_OVF_INT_CLR_W {
        RXFIFO_OVF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn frm_err_int_clr(&mut self) -> FRM_ERR_INT_CLR_W {
        FRM_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn parity_err_int_clr(&mut self) -> PARITY_ERR_INT_CLR_W {
        PARITY_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn txfifo_empty_int_clr(&mut self) -> TXFIFO_EMPTY_INT_CLR_W {
        TXFIFO_EMPTY_INT_CLR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rxfifo_full_int_clr(&mut self) -> RXFIFO_FULL_INT_CLR_W {
        RXFIFO_FULL_INT_CLR_W { w: self }
    }
}
