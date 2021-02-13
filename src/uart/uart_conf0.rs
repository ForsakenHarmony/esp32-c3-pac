#[doc = "Reader of register UART_CONF0"]
pub type R = crate::R<u32, super::UART_CONF0>;
#[doc = "Writer for register UART_CONF0"]
pub type W = crate::W<u32, super::UART_CONF0>;
#[doc = "Register UART_CONF0 `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_CONF0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_MEM_CLK_EN`"]
pub type UART_MEM_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_MEM_CLK_EN`"]
pub struct UART_MEM_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_MEM_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `UART_AUTOBAUD_EN`"]
pub type UART_AUTOBAUD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_AUTOBAUD_EN`"]
pub struct UART_AUTOBAUD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_AUTOBAUD_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `UART_ERR_WR_MASK`"]
pub type UART_ERR_WR_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_ERR_WR_MASK`"]
pub struct UART_ERR_WR_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_ERR_WR_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `UART_CLK_EN`"]
pub type UART_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_CLK_EN`"]
pub struct UART_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `UART_DTR_INV`"]
pub type UART_DTR_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_DTR_INV`"]
pub struct UART_DTR_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_DTR_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `UART_RTS_INV`"]
pub type UART_RTS_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RTS_INV`"]
pub struct UART_RTS_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RTS_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `UART_TXD_INV`"]
pub type UART_TXD_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_TXD_INV`"]
pub struct UART_TXD_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_TXD_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `UART_DSR_INV`"]
pub type UART_DSR_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_DSR_INV`"]
pub struct UART_DSR_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_DSR_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `UART_CTS_INV`"]
pub type UART_CTS_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_CTS_INV`"]
pub struct UART_CTS_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_CTS_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `UART_RXD_INV`"]
pub type UART_RXD_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RXD_INV`"]
pub struct UART_RXD_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RXD_INV_W<'a> {
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
#[doc = "Reader of field `UART_TXFIFO_RST`"]
pub type UART_TXFIFO_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_TXFIFO_RST`"]
pub struct UART_TXFIFO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_TXFIFO_RST_W<'a> {
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
#[doc = "Reader of field `UART_RXFIFO_RST`"]
pub type UART_RXFIFO_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RXFIFO_RST`"]
pub struct UART_RXFIFO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RXFIFO_RST_W<'a> {
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
#[doc = "Reader of field `UART_IRDA_EN`"]
pub type UART_IRDA_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_IRDA_EN`"]
pub struct UART_IRDA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_IRDA_EN_W<'a> {
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
#[doc = "Reader of field `UART_TX_FLOW_EN`"]
pub type UART_TX_FLOW_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_TX_FLOW_EN`"]
pub struct UART_TX_FLOW_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_TX_FLOW_EN_W<'a> {
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
#[doc = "Reader of field `UART_LOOPBACK`"]
pub type UART_LOOPBACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_LOOPBACK`"]
pub struct UART_LOOPBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_LOOPBACK_W<'a> {
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
#[doc = "Reader of field `UART_IRDA_RX_INV`"]
pub type UART_IRDA_RX_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_IRDA_RX_INV`"]
pub struct UART_IRDA_RX_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_IRDA_RX_INV_W<'a> {
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
#[doc = "Reader of field `UART_IRDA_TX_INV`"]
pub type UART_IRDA_TX_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_IRDA_TX_INV`"]
pub struct UART_IRDA_TX_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_IRDA_TX_INV_W<'a> {
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
#[doc = "Reader of field `UART_IRDA_WCTL`"]
pub type UART_IRDA_WCTL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_IRDA_WCTL`"]
pub struct UART_IRDA_WCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_IRDA_WCTL_W<'a> {
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
#[doc = "Reader of field `UART_IRDA_TX_EN`"]
pub type UART_IRDA_TX_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_IRDA_TX_EN`"]
pub struct UART_IRDA_TX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_IRDA_TX_EN_W<'a> {
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
#[doc = "Reader of field `UART_IRDA_DPLX`"]
pub type UART_IRDA_DPLX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_IRDA_DPLX`"]
pub struct UART_IRDA_DPLX_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_IRDA_DPLX_W<'a> {
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
#[doc = "Reader of field `UART_TXD_BRK`"]
pub type UART_TXD_BRK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_TXD_BRK`"]
pub struct UART_TXD_BRK_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_TXD_BRK_W<'a> {
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
#[doc = "Reader of field `UART_SW_DTR`"]
pub type UART_SW_DTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_SW_DTR`"]
pub struct UART_SW_DTR_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SW_DTR_W<'a> {
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
#[doc = "Reader of field `UART_SW_RTS`"]
pub type UART_SW_RTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_SW_RTS`"]
pub struct UART_SW_RTS_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SW_RTS_W<'a> {
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
#[doc = "Reader of field `UART_STOP_BIT_NUM`"]
pub type UART_STOP_BIT_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART_STOP_BIT_NUM`"]
pub struct UART_STOP_BIT_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_STOP_BIT_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `UART_BIT_NUM`"]
pub type UART_BIT_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART_BIT_NUM`"]
pub struct UART_BIT_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_BIT_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `UART_PARITY_EN`"]
pub type UART_PARITY_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_PARITY_EN`"]
pub struct UART_PARITY_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_PARITY_EN_W<'a> {
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
#[doc = "Reader of field `UART_PARITY`"]
pub type UART_PARITY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_PARITY`"]
pub struct UART_PARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_PARITY_W<'a> {
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
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn uart_mem_clk_en(&self) -> UART_MEM_CLK_EN_R {
        UART_MEM_CLK_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn uart_autobaud_en(&self) -> UART_AUTOBAUD_EN_R {
        UART_AUTOBAUD_EN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn uart_err_wr_mask(&self) -> UART_ERR_WR_MASK_R {
        UART_ERR_WR_MASK_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn uart_clk_en(&self) -> UART_CLK_EN_R {
        UART_CLK_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn uart_dtr_inv(&self) -> UART_DTR_INV_R {
        UART_DTR_INV_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn uart_rts_inv(&self) -> UART_RTS_INV_R {
        UART_RTS_INV_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn uart_txd_inv(&self) -> UART_TXD_INV_R {
        UART_TXD_INV_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn uart_dsr_inv(&self) -> UART_DSR_INV_R {
        UART_DSR_INV_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn uart_cts_inv(&self) -> UART_CTS_INV_R {
        UART_CTS_INV_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn uart_rxd_inv(&self) -> UART_RXD_INV_R {
        UART_RXD_INV_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn uart_txfifo_rst(&self) -> UART_TXFIFO_RST_R {
        UART_TXFIFO_RST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn uart_rxfifo_rst(&self) -> UART_RXFIFO_RST_R {
        UART_RXFIFO_RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn uart_irda_en(&self) -> UART_IRDA_EN_R {
        UART_IRDA_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn uart_tx_flow_en(&self) -> UART_TX_FLOW_EN_R {
        UART_TX_FLOW_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn uart_loopback(&self) -> UART_LOOPBACK_R {
        UART_LOOPBACK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn uart_irda_rx_inv(&self) -> UART_IRDA_RX_INV_R {
        UART_IRDA_RX_INV_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn uart_irda_tx_inv(&self) -> UART_IRDA_TX_INV_R {
        UART_IRDA_TX_INV_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn uart_irda_wctl(&self) -> UART_IRDA_WCTL_R {
        UART_IRDA_WCTL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn uart_irda_tx_en(&self) -> UART_IRDA_TX_EN_R {
        UART_IRDA_TX_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn uart_irda_dplx(&self) -> UART_IRDA_DPLX_R {
        UART_IRDA_DPLX_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn uart_txd_brk(&self) -> UART_TXD_BRK_R {
        UART_TXD_BRK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn uart_sw_dtr(&self) -> UART_SW_DTR_R {
        UART_SW_DTR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn uart_sw_rts(&self) -> UART_SW_RTS_R {
        UART_SW_RTS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn uart_stop_bit_num(&self) -> UART_STOP_BIT_NUM_R {
        UART_STOP_BIT_NUM_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn uart_bit_num(&self) -> UART_BIT_NUM_R {
        UART_BIT_NUM_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn uart_parity_en(&self) -> UART_PARITY_EN_R {
        UART_PARITY_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uart_parity(&self) -> UART_PARITY_R {
        UART_PARITY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn uart_mem_clk_en(&mut self) -> UART_MEM_CLK_EN_W {
        UART_MEM_CLK_EN_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn uart_autobaud_en(&mut self) -> UART_AUTOBAUD_EN_W {
        UART_AUTOBAUD_EN_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn uart_err_wr_mask(&mut self) -> UART_ERR_WR_MASK_W {
        UART_ERR_WR_MASK_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn uart_clk_en(&mut self) -> UART_CLK_EN_W {
        UART_CLK_EN_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn uart_dtr_inv(&mut self) -> UART_DTR_INV_W {
        UART_DTR_INV_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn uart_rts_inv(&mut self) -> UART_RTS_INV_W {
        UART_RTS_INV_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn uart_txd_inv(&mut self) -> UART_TXD_INV_W {
        UART_TXD_INV_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn uart_dsr_inv(&mut self) -> UART_DSR_INV_W {
        UART_DSR_INV_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn uart_cts_inv(&mut self) -> UART_CTS_INV_W {
        UART_CTS_INV_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn uart_rxd_inv(&mut self) -> UART_RXD_INV_W {
        UART_RXD_INV_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn uart_txfifo_rst(&mut self) -> UART_TXFIFO_RST_W {
        UART_TXFIFO_RST_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn uart_rxfifo_rst(&mut self) -> UART_RXFIFO_RST_W {
        UART_RXFIFO_RST_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn uart_irda_en(&mut self) -> UART_IRDA_EN_W {
        UART_IRDA_EN_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn uart_tx_flow_en(&mut self) -> UART_TX_FLOW_EN_W {
        UART_TX_FLOW_EN_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn uart_loopback(&mut self) -> UART_LOOPBACK_W {
        UART_LOOPBACK_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn uart_irda_rx_inv(&mut self) -> UART_IRDA_RX_INV_W {
        UART_IRDA_RX_INV_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn uart_irda_tx_inv(&mut self) -> UART_IRDA_TX_INV_W {
        UART_IRDA_TX_INV_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn uart_irda_wctl(&mut self) -> UART_IRDA_WCTL_W {
        UART_IRDA_WCTL_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn uart_irda_tx_en(&mut self) -> UART_IRDA_TX_EN_W {
        UART_IRDA_TX_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn uart_irda_dplx(&mut self) -> UART_IRDA_DPLX_W {
        UART_IRDA_DPLX_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn uart_txd_brk(&mut self) -> UART_TXD_BRK_W {
        UART_TXD_BRK_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn uart_sw_dtr(&mut self) -> UART_SW_DTR_W {
        UART_SW_DTR_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn uart_sw_rts(&mut self) -> UART_SW_RTS_W {
        UART_SW_RTS_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn uart_stop_bit_num(&mut self) -> UART_STOP_BIT_NUM_W {
        UART_STOP_BIT_NUM_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn uart_bit_num(&mut self) -> UART_BIT_NUM_W {
        UART_BIT_NUM_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn uart_parity_en(&mut self) -> UART_PARITY_EN_W {
        UART_PARITY_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uart_parity(&mut self) -> UART_PARITY_W {
        UART_PARITY_W { w: self }
    }
}
