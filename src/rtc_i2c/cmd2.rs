#[doc = "Reader of register CMD2"]
pub type R = crate::R<u32, super::CMD2>;
#[doc = "Writer for register CMD2"]
pub type W = crate::W<u32, super::CMD2>;
#[doc = "Register CMD2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMMAND2_DONE`"]
pub type COMMAND2_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `COMMAND2`"]
pub type COMMAND2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COMMAND2`"]
pub struct COMMAND2_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND2_W<'a> {
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
    pub fn command2_done(&self) -> COMMAND2_DONE_R {
        COMMAND2_DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command2(&self) -> COMMAND2_R {
        COMMAND2_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command2(&mut self) -> COMMAND2_W {
        COMMAND2_W { w: self }
    }
}
