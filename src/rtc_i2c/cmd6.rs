#[doc = "Reader of register CMD6"]
pub type R = crate::R<u32, super::CMD6>;
#[doc = "Writer for register CMD6"]
pub type W = crate::W<u32, super::CMD6>;
#[doc = "Register CMD6 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMMAND6_DONE`"]
pub type COMMAND6_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `COMMAND6`"]
pub type COMMAND6_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COMMAND6`"]
pub struct COMMAND6_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND6_W<'a> {
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
    pub fn command6_done(&self) -> COMMAND6_DONE_R {
        COMMAND6_DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command6(&self) -> COMMAND6_R {
        COMMAND6_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command6(&mut self) -> COMMAND6_W {
        COMMAND6_W { w: self }
    }
}
