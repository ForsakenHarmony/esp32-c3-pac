#[doc = "Reader of register CMD12"]
pub type R = crate::R<u32, super::CMD12>;
#[doc = "Writer for register CMD12"]
pub type W = crate::W<u32, super::CMD12>;
#[doc = "Register CMD12 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD12 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMMAND12_DONE`"]
pub type COMMAND12_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `COMMAND12`"]
pub type COMMAND12_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COMMAND12`"]
pub struct COMMAND12_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND12_W<'a> {
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
    pub fn command12_done(&self) -> COMMAND12_DONE_R {
        COMMAND12_DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command12(&self) -> COMMAND12_R {
        COMMAND12_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command12(&mut self) -> COMMAND12_W {
        COMMAND12_W { w: self }
    }
}
