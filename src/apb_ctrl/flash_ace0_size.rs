#[doc = "Reader of register FLASH_ACE0_SIZE"]
pub type R = crate::R<u32, super::FLASH_ACE0_SIZE>;
#[doc = "Writer for register FLASH_ACE0_SIZE"]
pub type W = crate::W<u32, super::FLASH_ACE0_SIZE>;
#[doc = "Register FLASH_ACE0_SIZE `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASH_ACE0_SIZE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FLASH_ACE0_SIZE`"]
pub type FLASH_ACE0_SIZE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FLASH_ACE0_SIZE`"]
pub struct FLASH_ACE0_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_ACE0_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | ((value as u32) & 0x1fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn flash_ace0_size(&self) -> FLASH_ACE0_SIZE_R {
        FLASH_ACE0_SIZE_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn flash_ace0_size(&mut self) -> FLASH_ACE0_SIZE_W {
        FLASH_ACE0_SIZE_W { w: self }
    }
}
