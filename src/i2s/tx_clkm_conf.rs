#[doc = "Reader of register TX_CLKM_CONF"]
pub type R = crate::R<u32, super::TX_CLKM_CONF>;
#[doc = "Writer for register TX_CLKM_CONF"]
pub type W = crate::W<u32, super::TX_CLKM_CONF>;
#[doc = "Register TX_CLKM_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::TX_CLKM_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `TX_CLK_SEL`"]
pub type TX_CLK_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_CLK_SEL`"]
pub struct TX_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
        self.w
    }
}
#[doc = "Reader of field `TX_CLK_ACTIVE`"]
pub type TX_CLK_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_CLK_ACTIVE`"]
pub struct TX_CLK_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CLK_ACTIVE_W<'a> {
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
#[doc = "Reader of field `TX_CLKM_DIV_NUM`"]
pub type TX_CLKM_DIV_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_CLKM_DIV_NUM`"]
pub struct TX_CLKM_DIV_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CLKM_DIV_NUM_W<'a> {
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
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn tx_clk_sel(&self) -> TX_CLK_SEL_R {
        TX_CLK_SEL_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tx_clk_active(&self) -> TX_CLK_ACTIVE_R {
        TX_CLK_ACTIVE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn tx_clkm_div_num(&self) -> TX_CLKM_DIV_NUM_R {
        TX_CLKM_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W {
        CLK_EN_W { w: self }
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn tx_clk_sel(&mut self) -> TX_CLK_SEL_W {
        TX_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tx_clk_active(&mut self) -> TX_CLK_ACTIVE_W {
        TX_CLK_ACTIVE_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn tx_clkm_div_num(&mut self) -> TX_CLKM_DIV_NUM_W {
        TX_CLKM_DIV_NUM_W { w: self }
    }
}
