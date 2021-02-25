#[doc = "Reader of register FLASH_ACE0_ATTR"]
pub type R = crate::R<u32, super::FLASH_ACE0_ATTR>;
#[doc = "Writer for register FLASH_ACE0_ATTR"]
pub type W = crate::W<u32, super::FLASH_ACE0_ATTR>;
#[doc = "Register FLASH_ACE0_ATTR `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASH_ACE0_ATTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FLASH_ACE0_ATTR`"]
pub type FLASH_ACE0_ATTR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FLASH_ACE0_ATTR`"]
pub struct FLASH_ACE0_ATTR_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_ACE0_ATTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn flash_ace0_attr(&self) -> FLASH_ACE0_ATTR_R {
        FLASH_ACE0_ATTR_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn flash_ace0_attr(&mut self) -> FLASH_ACE0_ATTR_W {
        FLASH_ACE0_ATTR_W { w: self }
    }
}
