#[doc = "Reader of register CMD9"]
pub type R = crate::R<u32, super::CMD9>;
#[doc = "Writer for register CMD9"]
pub type W = crate::W<u32, super::CMD9>;
#[doc = "Register CMD9 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD9 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMMAND9_DONE`"]
pub type COMMAND9_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `COMMAND9`"]
pub type COMMAND9_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COMMAND9`"]
pub struct COMMAND9_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND9_W<'a> {
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
    pub fn command9_done(&self) -> COMMAND9_DONE_R {
        COMMAND9_DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command9(&self) -> COMMAND9_R {
        COMMAND9_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command9(&mut self) -> COMMAND9_W {
        COMMAND9_W { w: self }
    }
}
