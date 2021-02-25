#[doc = "Reader of register CMD8"]
pub type R = crate::R<u32, super::CMD8>;
#[doc = "Writer for register CMD8"]
pub type W = crate::W<u32, super::CMD8>;
#[doc = "Register CMD8 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD8 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMMAND8_DONE`"]
pub type COMMAND8_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `COMMAND8`"]
pub type COMMAND8_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COMMAND8`"]
pub struct COMMAND8_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn command8_done(&self) -> COMMAND8_DONE_R {
        COMMAND8_DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command8(&self) -> COMMAND8_R {
        COMMAND8_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command8(&mut self) -> COMMAND8_W {
        COMMAND8_W { w: self }
    }
}
