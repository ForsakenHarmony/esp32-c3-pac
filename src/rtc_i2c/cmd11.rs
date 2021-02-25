#[doc = "Reader of register CMD11"]
pub type R = crate::R<u32, super::CMD11>;
#[doc = "Writer for register CMD11"]
pub type W = crate::W<u32, super::CMD11>;
#[doc = "Register CMD11 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD11 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMMAND11_DONE`"]
pub type COMMAND11_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `COMMAND11`"]
pub type COMMAND11_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COMMAND11`"]
pub struct COMMAND11_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND11_W<'a> {
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
    pub fn command11_done(&self) -> COMMAND11_DONE_R {
        COMMAND11_DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command11(&self) -> COMMAND11_R {
        COMMAND11_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command11(&mut self) -> COMMAND11_W {
        COMMAND11_W { w: self }
    }
}
