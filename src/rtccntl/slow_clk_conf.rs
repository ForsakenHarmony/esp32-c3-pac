#[doc = "Reader of register SLOW_CLK_CONF"]
pub type R = crate::R<u32, super::SLOW_CLK_CONF>;
#[doc = "Writer for register SLOW_CLK_CONF"]
pub type W = crate::W<u32, super::SLOW_CLK_CONF>;
#[doc = "Register SLOW_CLK_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::SLOW_CLK_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLOW_CLK_NEXT_EDGE`"]
pub type SLOW_CLK_NEXT_EDGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLOW_CLK_NEXT_EDGE`"]
pub struct SLOW_CLK_NEXT_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOW_CLK_NEXT_EDGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `ANA_CLK_DIV`"]
pub type ANA_CLK_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ANA_CLK_DIV`"]
pub struct ANA_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ANA_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 23)) | (((value as u32) & 0xff) << 23);
        self.w
    }
}
#[doc = "Reader of field `ANA_CLK_DIV_VLD`"]
pub type ANA_CLK_DIV_VLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ANA_CLK_DIV_VLD`"]
pub struct ANA_CLK_DIV_VLD_W<'a> {
    w: &'a mut W,
}
impl<'a> ANA_CLK_DIV_VLD_W<'a> {
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
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn slow_clk_next_edge(&self) -> SLOW_CLK_NEXT_EDGE_R {
        SLOW_CLK_NEXT_EDGE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 23:30"]
    #[inline(always)]
    pub fn ana_clk_div(&self) -> ANA_CLK_DIV_R {
        ANA_CLK_DIV_R::new(((self.bits >> 23) & 0xff) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ana_clk_div_vld(&self) -> ANA_CLK_DIV_VLD_R {
        ANA_CLK_DIV_VLD_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn slow_clk_next_edge(&mut self) -> SLOW_CLK_NEXT_EDGE_W {
        SLOW_CLK_NEXT_EDGE_W { w: self }
    }
    #[doc = "Bits 23:30"]
    #[inline(always)]
    pub fn ana_clk_div(&mut self) -> ANA_CLK_DIV_W {
        ANA_CLK_DIV_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ana_clk_div_vld(&mut self) -> ANA_CLK_DIV_VLD_W {
        ANA_CLK_DIV_VLD_W { w: self }
    }
}
