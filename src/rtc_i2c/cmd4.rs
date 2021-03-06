#[doc = "Reader of register CMD4"]
pub type R = crate::R<u32, super::CMD4>;
#[doc = "Writer for register CMD4"]
pub type W = crate::W<u32, super::CMD4>;
#[doc = "Register CMD4 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMMAND4_DONE`"]
pub type COMMAND4_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `COMMAND4`"]
pub type COMMAND4_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COMMAND4`"]
pub struct COMMAND4_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND4_W<'a> {
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
    pub fn command4_done(&self) -> COMMAND4_DONE_R {
        COMMAND4_DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command4(&self) -> COMMAND4_R {
        COMMAND4_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command4(&mut self) -> COMMAND4_W {
        COMMAND4_W { w: self }
    }
}
