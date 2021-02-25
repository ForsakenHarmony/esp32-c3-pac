#[doc = "Reader of register CONF0"]
pub type R = crate::R<u32, super::CONF0>;
#[doc = "Writer for register CONF0"]
pub type W = crate::W<u32, super::CONF0>;
#[doc = "Register CONF0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CONF0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_RX_BRK_EOF_EN`"]
pub type UART_RX_BRK_EOF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RX_BRK_EOF_EN`"]
pub struct UART_RX_BRK_EOF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RX_BRK_EOF_EN_W<'a> {
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
#[doc = "Reader of field `CLK_EN`"]
pub type CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK_EN`"]
pub struct CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_EN_W<'a> {
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
#[doc = "Reader of field `ENCODE_CRC_EN`"]
pub type ENCODE_CRC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENCODE_CRC_EN`"]
pub struct ENCODE_CRC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ENCODE_CRC_EN_W<'a> {
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
#[doc = "Reader of field `LEN_EOF_EN`"]
pub type LEN_EOF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEN_EOF_EN`"]
pub struct LEN_EOF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEN_EOF_EN_W<'a> {
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
#[doc = "Reader of field `UART_IDLE_EOF_EN`"]
pub type UART_IDLE_EOF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_IDLE_EOF_EN`"]
pub struct UART_IDLE_EOF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_IDLE_EOF_EN_W<'a> {
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
#[doc = "Reader of field `CRC_REC_EN`"]
pub type CRC_REC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRC_REC_EN`"]
pub struct CRC_REC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_REC_EN_W<'a> {
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
#[doc = "Reader of field `HEAD_EN`"]
pub type HEAD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HEAD_EN`"]
pub struct HEAD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HEAD_EN_W<'a> {
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
#[doc = "Reader of field `SEPER_EN`"]
pub type SEPER_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEPER_EN`"]
pub struct SEPER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SEPER_EN_W<'a> {
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
#[doc = "Reader of field `UART1_CE`"]
pub type UART1_CE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART1_CE`"]
pub struct UART1_CE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1_CE_W<'a> {
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
#[doc = "Reader of field `UART0_CE`"]
pub type UART0_CE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART0_CE`"]
pub struct UART0_CE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0_CE_W<'a> {
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
#[doc = "Reader of field `RX_RST`"]
pub type RX_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_RST`"]
pub struct RX_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_RST_W<'a> {
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
#[doc = "Reader of field `TX_RST`"]
pub type TX_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_RST`"]
pub struct TX_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_RST_W<'a> {
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
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn uart_rx_brk_eof_en(&self) -> UART_RX_BRK_EOF_EN_R {
        UART_RX_BRK_EOF_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn encode_crc_en(&self) -> ENCODE_CRC_EN_R {
        ENCODE_CRC_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn len_eof_en(&self) -> LEN_EOF_EN_R {
        LEN_EOF_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn uart_idle_eof_en(&self) -> UART_IDLE_EOF_EN_R {
        UART_IDLE_EOF_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn crc_rec_en(&self) -> CRC_REC_EN_R {
        CRC_REC_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn head_en(&self) -> HEAD_EN_R {
        HEAD_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn seper_en(&self) -> SEPER_EN_R {
        SEPER_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn uart1_ce(&self) -> UART1_CE_R {
        UART1_CE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn uart0_ce(&self) -> UART0_CE_R {
        UART0_CE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rx_rst(&self) -> RX_RST_R {
        RX_RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_rst(&self) -> TX_RST_R {
        TX_RST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn uart_rx_brk_eof_en(&mut self) -> UART_RX_BRK_EOF_EN_W {
        UART_RX_BRK_EOF_EN_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W {
        CLK_EN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn encode_crc_en(&mut self) -> ENCODE_CRC_EN_W {
        ENCODE_CRC_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn len_eof_en(&mut self) -> LEN_EOF_EN_W {
        LEN_EOF_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn uart_idle_eof_en(&mut self) -> UART_IDLE_EOF_EN_W {
        UART_IDLE_EOF_EN_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn crc_rec_en(&mut self) -> CRC_REC_EN_W {
        CRC_REC_EN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn head_en(&mut self) -> HEAD_EN_W {
        HEAD_EN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn seper_en(&mut self) -> SEPER_EN_W {
        SEPER_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn uart1_ce(&mut self) -> UART1_CE_W {
        UART1_CE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn uart0_ce(&mut self) -> UART0_CE_W {
        UART0_CE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rx_rst(&mut self) -> RX_RST_W {
        RX_RST_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_rst(&mut self) -> TX_RST_W {
        TX_RST_W { w: self }
    }
}
