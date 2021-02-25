#[doc = "Reader of register CLKGATE_FORCE_ON"]
pub type R = crate::R<u32, super::CLKGATE_FORCE_ON>;
#[doc = "Writer for register CLKGATE_FORCE_ON"]
pub type W = crate::W<u32, super::CLKGATE_FORCE_ON>;
#[doc = "Register CLKGATE_FORCE_ON `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKGATE_FORCE_ON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SRAM_CLKGATE_FORCE_ON`"]
pub type SRAM_CLKGATE_FORCE_ON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SRAM_CLKGATE_FORCE_ON`"]
pub struct SRAM_CLKGATE_FORCE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_CLKGATE_FORCE_ON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
#[doc = "Reader of field `ROM_CLKGATE_FORCE_ON`"]
pub type ROM_CLKGATE_FORCE_ON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ROM_CLKGATE_FORCE_ON`"]
pub struct ROM_CLKGATE_FORCE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_CLKGATE_FORCE_ON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn sram_clkgate_force_on(&self) -> SRAM_CLKGATE_FORCE_ON_R {
        SRAM_CLKGATE_FORCE_ON_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rom_clkgate_force_on(&self) -> ROM_CLKGATE_FORCE_ON_R {
        ROM_CLKGATE_FORCE_ON_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn sram_clkgate_force_on(&mut self) -> SRAM_CLKGATE_FORCE_ON_W {
        SRAM_CLKGATE_FORCE_ON_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rom_clkgate_force_on(&mut self) -> ROM_CLKGATE_FORCE_ON_W {
        ROM_CLKGATE_FORCE_ON_W { w: self }
    }
}
