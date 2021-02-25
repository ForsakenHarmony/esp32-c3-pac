#[doc = "Reader of register CLK_CONF"]
pub type R = crate::R<u32, super::CLK_CONF>;
#[doc = "Writer for register CLK_CONF"]
pub type W = crate::W<u32, super::CLK_CONF>;
#[doc = "Register CLK_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RX_RST_CORE`"]
pub type RX_RST_CORE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_RST_CORE`"]
pub struct RX_RST_CORE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_RST_CORE_W<'a> {
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
#[doc = "Reader of field `TX_RST_CORE`"]
pub type TX_RST_CORE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_RST_CORE`"]
pub struct TX_RST_CORE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_RST_CORE_W<'a> {
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
#[doc = "Reader of field `RX_SCLK_EN`"]
pub type RX_SCLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_SCLK_EN`"]
pub struct RX_SCLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_SCLK_EN_W<'a> {
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
#[doc = "Reader of field `TX_SCLK_EN`"]
pub type TX_SCLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_SCLK_EN`"]
pub struct TX_SCLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_SCLK_EN_W<'a> {
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
#[doc = "Reader of field `RST_CORE`"]
pub type RST_CORE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RST_CORE`"]
pub struct RST_CORE_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_CORE_W<'a> {
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
#[doc = "Reader of field `SCLK_EN`"]
pub type SCLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCLK_EN`"]
pub struct SCLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_EN_W<'a> {
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
#[doc = "Reader of field `SCLK_SEL`"]
pub type SCLK_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCLK_SEL`"]
pub struct SCLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `SCLK_DIV_NUM`"]
pub type SCLK_DIV_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCLK_DIV_NUM`"]
pub struct SCLK_DIV_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_DIV_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 12)) | (((value as u32) & 0xff) << 12);
        self.w
    }
}
#[doc = "Reader of field `SCLK_DIV_A`"]
pub type SCLK_DIV_A_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCLK_DIV_A`"]
pub struct SCLK_DIV_A_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_DIV_A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | (((value as u32) & 0x3f) << 6);
        self.w
    }
}
#[doc = "Reader of field `SCLK_DIV_B`"]
pub type SCLK_DIV_B_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCLK_DIV_B`"]
pub struct SCLK_DIV_B_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_DIV_B_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rx_rst_core(&self) -> RX_RST_CORE_R {
        RX_RST_CORE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tx_rst_core(&self) -> TX_RST_CORE_R {
        TX_RST_CORE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rx_sclk_en(&self) -> RX_SCLK_EN_R {
        RX_SCLK_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tx_sclk_en(&self) -> TX_SCLK_EN_R {
        TX_SCLK_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rst_core(&self) -> RST_CORE_R {
        RST_CORE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn sclk_en(&self) -> SCLK_EN_R {
        SCLK_EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn sclk_sel(&self) -> SCLK_SEL_R {
        SCLK_SEL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 12:19"]
    #[inline(always)]
    pub fn sclk_div_num(&self) -> SCLK_DIV_NUM_R {
        SCLK_DIV_NUM_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn sclk_div_a(&self) -> SCLK_DIV_A_R {
        SCLK_DIV_A_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn sclk_div_b(&self) -> SCLK_DIV_B_R {
        SCLK_DIV_B_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rx_rst_core(&mut self) -> RX_RST_CORE_W {
        RX_RST_CORE_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tx_rst_core(&mut self) -> TX_RST_CORE_W {
        TX_RST_CORE_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rx_sclk_en(&mut self) -> RX_SCLK_EN_W {
        RX_SCLK_EN_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tx_sclk_en(&mut self) -> TX_SCLK_EN_W {
        TX_SCLK_EN_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rst_core(&mut self) -> RST_CORE_W {
        RST_CORE_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn sclk_en(&mut self) -> SCLK_EN_W {
        SCLK_EN_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn sclk_sel(&mut self) -> SCLK_SEL_W {
        SCLK_SEL_W { w: self }
    }
    #[doc = "Bits 12:19"]
    #[inline(always)]
    pub fn sclk_div_num(&mut self) -> SCLK_DIV_NUM_W {
        SCLK_DIV_NUM_W { w: self }
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn sclk_div_a(&mut self) -> SCLK_DIV_A_W {
        SCLK_DIV_A_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn sclk_div_b(&mut self) -> SCLK_DIV_B_W {
        SCLK_DIV_B_W { w: self }
    }
}
