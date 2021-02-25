#[doc = "Reader of register CMD7"]
pub type R = crate::R<u32, super::CMD7>;
#[doc = "Writer for register CMD7"]
pub type W = crate::W<u32, super::CMD7>;
#[doc = "Register CMD7 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMMAND7_DONE`"]
pub type COMMAND7_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `COMMAND7`"]
pub type COMMAND7_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COMMAND7`"]
pub struct COMMAND7_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND7_W<'a> {
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
    pub fn command7_done(&self) -> COMMAND7_DONE_R {
        COMMAND7_DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command7(&self) -> COMMAND7_R {
        COMMAND7_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command7(&mut self) -> COMMAND7_W {
        COMMAND7_W { w: self }
    }
}
