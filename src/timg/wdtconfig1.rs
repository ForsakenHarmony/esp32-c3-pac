#[doc = "Reader of register WDTCONFIG1"]
pub type R = crate::R<u32, super::WDTCONFIG1>;
#[doc = "Writer for register WDTCONFIG1"]
pub type W = crate::W<u32, super::WDTCONFIG1>;
#[doc = "Register WDTCONFIG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::WDTCONFIG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDT_CLK_PRESCALE`"]
pub type WDT_CLK_PRESCALE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WDT_CLK_PRESCALE`"]
pub struct WDT_CLK_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_CLK_PRESCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Write proxy for field `WDT_DIVCNT_RST`"]
pub struct WDT_DIVCNT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_DIVCNT_RST_W<'a> {
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
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn wdt_clk_prescale(&self) -> WDT_CLK_PRESCALE_R {
        WDT_CLK_PRESCALE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn wdt_clk_prescale(&mut self) -> WDT_CLK_PRESCALE_W {
        WDT_CLK_PRESCALE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wdt_divcnt_rst(&mut self) -> WDT_DIVCNT_RST_W {
        WDT_DIVCNT_RST_W { w: self }
    }
}
