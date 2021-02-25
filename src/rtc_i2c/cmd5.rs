#[doc = "Reader of register CMD5"]
pub type R = crate::R<u32, super::CMD5>;
#[doc = "Writer for register CMD5"]
pub type W = crate::W<u32, super::CMD5>;
#[doc = "Register CMD5 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMMAND5_DONE`"]
pub type COMMAND5_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `COMMAND5`"]
pub type COMMAND5_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COMMAND5`"]
pub struct COMMAND5_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND5_W<'a> {
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
    pub fn command5_done(&self) -> COMMAND5_DONE_R {
        COMMAND5_DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command5(&self) -> COMMAND5_R {
        COMMAND5_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command5(&mut self) -> COMMAND5_W {
        COMMAND5_W { w: self }
    }
}
