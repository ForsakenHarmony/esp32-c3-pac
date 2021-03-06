#[doc = "Reader of register CMD14"]
pub type R = crate::R<u32, super::CMD14>;
#[doc = "Writer for register CMD14"]
pub type W = crate::W<u32, super::CMD14>;
#[doc = "Register CMD14 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD14 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMMAND14_DONE`"]
pub type COMMAND14_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `COMMAND14`"]
pub type COMMAND14_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COMMAND14`"]
pub struct COMMAND14_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND14_W<'a> {
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
    pub fn command14_done(&self) -> COMMAND14_DONE_R {
        COMMAND14_DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command14(&self) -> COMMAND14_R {
        COMMAND14_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command14(&mut self) -> COMMAND14_W {
        COMMAND14_W { w: self }
    }
}
