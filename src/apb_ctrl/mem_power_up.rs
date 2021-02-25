#[doc = "Reader of register MEM_POWER_UP"]
pub type R = crate::R<u32, super::MEM_POWER_UP>;
#[doc = "Writer for register MEM_POWER_UP"]
pub type W = crate::W<u32, super::MEM_POWER_UP>;
#[doc = "Register MEM_POWER_UP `reset()`'s with value 0"]
impl crate::ResetValue for super::MEM_POWER_UP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SRAM_POWER_UP`"]
pub type SRAM_POWER_UP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SRAM_POWER_UP`"]
pub struct SRAM_POWER_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_POWER_UP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
#[doc = "Reader of field `ROM_POWER_UP`"]
pub type ROM_POWER_UP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ROM_POWER_UP`"]
pub struct ROM_POWER_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_POWER_UP_W<'a> {
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
    pub fn sram_power_up(&self) -> SRAM_POWER_UP_R {
        SRAM_POWER_UP_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rom_power_up(&self) -> ROM_POWER_UP_R {
        ROM_POWER_UP_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn sram_power_up(&mut self) -> SRAM_POWER_UP_W {
        SRAM_POWER_UP_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rom_power_up(&mut self) -> ROM_POWER_UP_W {
        ROM_POWER_UP_W { w: self }
    }
}
