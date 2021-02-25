#[doc = "Reader of register CMD1"]
pub type R = crate::R<u32, super::CMD1>;
#[doc = "Writer for register CMD1"]
pub type W = crate::W<u32, super::CMD1>;
#[doc = "Register CMD1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMMAND1_DONE`"]
pub type COMMAND1_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `COMMAND1`"]
pub type COMMAND1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COMMAND1`"]
pub struct COMMAND1_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND1_W<'a> {
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
    pub fn command1_done(&self) -> COMMAND1_DONE_R {
        COMMAND1_DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command1(&self) -> COMMAND1_R {
        COMMAND1_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command1(&mut self) -> COMMAND1_W {
        COMMAND1_W { w: self }
    }
}
