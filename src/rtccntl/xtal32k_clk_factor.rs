#[doc = "Reader of register XTAL32K_CLK_FACTOR"]
pub type R = crate::R<u32, super::XTAL32K_CLK_FACTOR>;
#[doc = "Writer for register XTAL32K_CLK_FACTOR"]
pub type W = crate::W<u32, super::XTAL32K_CLK_FACTOR>;
#[doc = "Register XTAL32K_CLK_FACTOR `reset()`'s with value 0"]
impl crate::ResetValue for super::XTAL32K_CLK_FACTOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XTAL32K_CLK_FACTOR`"]
pub type XTAL32K_CLK_FACTOR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `XTAL32K_CLK_FACTOR`"]
pub struct XTAL32K_CLK_FACTOR_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_CLK_FACTOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn xtal32k_clk_factor(&self) -> XTAL32K_CLK_FACTOR_R {
        XTAL32K_CLK_FACTOR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn xtal32k_clk_factor(&mut self) -> XTAL32K_CLK_FACTOR_W {
        XTAL32K_CLK_FACTOR_W { w: self }
    }
}
