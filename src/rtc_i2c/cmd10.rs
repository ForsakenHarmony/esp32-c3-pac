#[doc = "Reader of register CMD10"]
pub type R = crate::R<u32, super::CMD10>;
#[doc = "Writer for register CMD10"]
pub type W = crate::W<u32, super::CMD10>;
#[doc = "Register CMD10 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD10 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMMAND10_DONE`"]
pub type COMMAND10_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `COMMAND10`"]
pub type COMMAND10_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COMMAND10`"]
pub struct COMMAND10_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND10_W<'a> {
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
    pub fn command10_done(&self) -> COMMAND10_DONE_R {
        COMMAND10_DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command10(&self) -> COMMAND10_R {
        COMMAND10_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command10(&mut self) -> COMMAND10_W {
        COMMAND10_W { w: self }
    }
}
