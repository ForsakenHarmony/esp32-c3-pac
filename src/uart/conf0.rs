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
#[doc = "Reader of field `MEM_CLK_EN`"]
pub type MEM_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MEM_CLK_EN`"]
pub struct MEM_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_CLK_EN_W<'a> {
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
#[doc = "Reader of field `AUTOBAUD_EN`"]
pub type AUTOBAUD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOBAUD_EN`"]
pub struct AUTOBAUD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOBAUD_EN_W<'a> {
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
#[doc = "Reader of field `ERR_WR_MASK`"]
pub type ERR_WR_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERR_WR_MASK`"]
pub struct ERR_WR_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_WR_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `DTR_INV`"]
pub type DTR_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTR_INV`"]
pub struct DTR_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> DTR_INV_W<'a> {
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
#[doc = "Reader of field `RTS_INV`"]
pub type RTS_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTS_INV`"]
pub struct RTS_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> RTS_INV_W<'a> {
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
#[doc = "Reader of field `TXD_INV`"]
pub type TXD_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXD_INV`"]
pub struct TXD_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> TXD_INV_W<'a> {
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
#[doc = "Reader of field `DSR_INV`"]
pub type DSR_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSR_INV`"]
pub struct DSR_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> DSR_INV_W<'a> {
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
#[doc = "Reader of field `CTS_INV`"]
pub type CTS_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTS_INV`"]
pub struct CTS_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> CTS_INV_W<'a> {
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
#[doc = "Reader of field `RXD_INV`"]
pub type RXD_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXD_INV`"]
pub struct RXD_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> RXD_INV_W<'a> {
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
#[doc = "Reader of field `TXFIFO_RST`"]
pub type TXFIFO_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFIFO_RST`"]
pub struct TXFIFO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFO_RST_W<'a> {
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
#[doc = "Reader of field `RXFIFO_RST`"]
pub type RXFIFO_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFIFO_RST`"]
pub struct RXFIFO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_RST_W<'a> {
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
#[doc = "Reader of field `IRDA_EN`"]
pub type IRDA_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRDA_EN`"]
pub struct IRDA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> IRDA_EN_W<'a> {
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
#[doc = "Reader of field `TX_FLOW_EN`"]
pub type TX_FLOW_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_FLOW_EN`"]
pub struct TX_FLOW_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FLOW_EN_W<'a> {
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
#[doc = "Reader of field `LOOPBACK`"]
pub type LOOPBACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOOPBACK`"]
pub struct LOOPBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPBACK_W<'a> {
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
#[doc = "Reader of field `IRDA_RX_INV`"]
pub type IRDA_RX_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRDA_RX_INV`"]
pub struct IRDA_RX_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> IRDA_RX_INV_W<'a> {
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
#[doc = "Reader of field `IRDA_TX_INV`"]
pub type IRDA_TX_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRDA_TX_INV`"]
pub struct IRDA_TX_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> IRDA_TX_INV_W<'a> {
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
#[doc = "Reader of field `IRDA_WCTL`"]
pub type IRDA_WCTL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRDA_WCTL`"]
pub struct IRDA_WCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> IRDA_WCTL_W<'a> {
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
#[doc = "Reader of field `IRDA_TX_EN`"]
pub type IRDA_TX_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRDA_TX_EN`"]
pub struct IRDA_TX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> IRDA_TX_EN_W<'a> {
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
#[doc = "Reader of field `IRDA_DPLX`"]
pub type IRDA_DPLX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRDA_DPLX`"]
pub struct IRDA_DPLX_W<'a> {
    w: &'a mut W,
}
impl<'a> IRDA_DPLX_W<'a> {
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
#[doc = "Reader of field `TXD_BRK`"]
pub type TXD_BRK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXD_BRK`"]
pub struct TXD_BRK_W<'a> {
    w: &'a mut W,
}
impl<'a> TXD_BRK_W<'a> {
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
#[doc = "Reader of field `SW_DTR`"]
pub type SW_DTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_DTR`"]
pub struct SW_DTR_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_DTR_W<'a> {
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
#[doc = "Reader of field `SW_RTS`"]
pub type SW_RTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_RTS`"]
pub struct SW_RTS_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_RTS_W<'a> {
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
#[doc = "Reader of field `STOP_BIT_NUM`"]
pub type STOP_BIT_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STOP_BIT_NUM`"]
pub struct STOP_BIT_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_BIT_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `BIT_NUM`"]
pub type BIT_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BIT_NUM`"]
pub struct BIT_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> BIT_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `PARITY_EN`"]
pub type PARITY_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PARITY_EN`"]
pub struct PARITY_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITY_EN_W<'a> {
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
#[doc = "Reader of field `PARITY`"]
pub type PARITY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PARITY`"]
pub struct PARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITY_W<'a> {
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
    pub fn mem_clk_en(&self) -> MEM_CLK_EN_R {
        MEM_CLK_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn autobaud_en(&self) -> AUTOBAUD_EN_R {
        AUTOBAUD_EN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn err_wr_mask(&self) -> ERR_WR_MASK_R {
        ERR_WR_MASK_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn dtr_inv(&self) -> DTR_INV_R {
        DTR_INV_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rts_inv(&self) -> RTS_INV_R {
        RTS_INV_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn txd_inv(&self) -> TXD_INV_R {
        TXD_INV_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn dsr_inv(&self) -> DSR_INV_R {
        DSR_INV_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cts_inv(&self) -> CTS_INV_R {
        CTS_INV_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rxd_inv(&self) -> RXD_INV_R {
        RXD_INV_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn txfifo_rst(&self) -> TXFIFO_RST_R {
        TXFIFO_RST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rxfifo_rst(&self) -> RXFIFO_RST_R {
        RXFIFO_RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn irda_en(&self) -> IRDA_EN_R {
        IRDA_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tx_flow_en(&self) -> TX_FLOW_EN_R {
        TX_FLOW_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn irda_rx_inv(&self) -> IRDA_RX_INV_R {
        IRDA_RX_INV_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn irda_tx_inv(&self) -> IRDA_TX_INV_R {
        IRDA_TX_INV_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn irda_wctl(&self) -> IRDA_WCTL_R {
        IRDA_WCTL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn irda_tx_en(&self) -> IRDA_TX_EN_R {
        IRDA_TX_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn irda_dplx(&self) -> IRDA_DPLX_R {
        IRDA_DPLX_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn txd_brk(&self) -> TXD_BRK_R {
        TXD_BRK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sw_dtr(&self) -> SW_DTR_R {
        SW_DTR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sw_rts(&self) -> SW_RTS_R {
        SW_RTS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn stop_bit_num(&self) -> STOP_BIT_NUM_R {
        STOP_BIT_NUM_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn bit_num(&self) -> BIT_NUM_R {
        BIT_NUM_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn parity_en(&self) -> PARITY_EN_R {
        PARITY_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn mem_clk_en(&mut self) -> MEM_CLK_EN_W {
        MEM_CLK_EN_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn autobaud_en(&mut self) -> AUTOBAUD_EN_W {
        AUTOBAUD_EN_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn err_wr_mask(&mut self) -> ERR_WR_MASK_W {
        ERR_WR_MASK_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W {
        CLK_EN_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn dtr_inv(&mut self) -> DTR_INV_W {
        DTR_INV_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rts_inv(&mut self) -> RTS_INV_W {
        RTS_INV_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn txd_inv(&mut self) -> TXD_INV_W {
        TXD_INV_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn dsr_inv(&mut self) -> DSR_INV_W {
        DSR_INV_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cts_inv(&mut self) -> CTS_INV_W {
        CTS_INV_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rxd_inv(&mut self) -> RXD_INV_W {
        RXD_INV_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn txfifo_rst(&mut self) -> TXFIFO_RST_W {
        TXFIFO_RST_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rxfifo_rst(&mut self) -> RXFIFO_RST_W {
        RXFIFO_RST_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn irda_en(&mut self) -> IRDA_EN_W {
        IRDA_EN_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tx_flow_en(&mut self) -> TX_FLOW_EN_W {
        TX_FLOW_EN_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn loopback(&mut self) -> LOOPBACK_W {
        LOOPBACK_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn irda_rx_inv(&mut self) -> IRDA_RX_INV_W {
        IRDA_RX_INV_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn irda_tx_inv(&mut self) -> IRDA_TX_INV_W {
        IRDA_TX_INV_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn irda_wctl(&mut self) -> IRDA_WCTL_W {
        IRDA_WCTL_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn irda_tx_en(&mut self) -> IRDA_TX_EN_W {
        IRDA_TX_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn irda_dplx(&mut self) -> IRDA_DPLX_W {
        IRDA_DPLX_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn txd_brk(&mut self) -> TXD_BRK_W {
        TXD_BRK_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sw_dtr(&mut self) -> SW_DTR_W {
        SW_DTR_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sw_rts(&mut self) -> SW_RTS_W {
        SW_RTS_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn stop_bit_num(&mut self) -> STOP_BIT_NUM_W {
        STOP_BIT_NUM_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn bit_num(&mut self) -> BIT_NUM_W {
        BIT_NUM_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn parity_en(&mut self) -> PARITY_EN_W {
        PARITY_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn parity(&mut self) -> PARITY_W {
        PARITY_W { w: self }
    }
}
