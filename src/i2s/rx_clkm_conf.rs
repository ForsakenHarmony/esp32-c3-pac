#[doc = "Reader of register RX_CLKM_CONF"]
pub type R = crate::R<u32, super::RX_CLKM_CONF>;
#[doc = "Writer for register RX_CLKM_CONF"]
pub type W = crate::W<u32, super::RX_CLKM_CONF>;
#[doc = "Register RX_CLKM_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::RX_CLKM_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCLK_SEL`"]
pub type MCLK_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCLK_SEL`"]
pub struct MCLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MCLK_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `RX_CLK_SEL`"]
pub type RX_CLK_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_CLK_SEL`"]
pub struct RX_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
        self.w
    }
}
#[doc = "Reader of field `RX_CLK_ACTIVE`"]
pub type RX_CLK_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_CLK_ACTIVE`"]
pub struct RX_CLK_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_CLK_ACTIVE_W<'a> {
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
#[doc = "Reader of field `RX_CLKM_DIV_NUM`"]
pub type RX_CLKM_DIV_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_CLKM_DIV_NUM`"]
pub struct RX_CLKM_DIV_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_CLKM_DIV_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn mclk_sel(&self) -> MCLK_SEL_R {
        MCLK_SEL_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn rx_clk_sel(&self) -> RX_CLK_SEL_R {
        RX_CLK_SEL_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rx_clk_active(&self) -> RX_CLK_ACTIVE_R {
        RX_CLK_ACTIVE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rx_clkm_div_num(&self) -> RX_CLKM_DIV_NUM_R {
        RX_CLKM_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn mclk_sel(&mut self) -> MCLK_SEL_W {
        MCLK_SEL_W { w: self }
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn rx_clk_sel(&mut self) -> RX_CLK_SEL_W {
        RX_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rx_clk_active(&mut self) -> RX_CLK_ACTIVE_W {
        RX_CLK_ACTIVE_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rx_clkm_div_num(&mut self) -> RX_CLKM_DIV_NUM_W {
        RX_CLKM_DIV_NUM_W { w: self }
    }
}
