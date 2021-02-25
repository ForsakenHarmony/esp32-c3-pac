#[doc = "Reader of register CMD15"]
pub type R = crate::R<u32, super::CMD15>;
#[doc = "Writer for register CMD15"]
pub type W = crate::W<u32, super::CMD15>;
#[doc = "Register CMD15 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD15 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMMAND15_DONE`"]
pub type COMMAND15_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `COMMAND15`"]
pub type COMMAND15_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COMMAND15`"]
pub struct COMMAND15_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND15_W<'a> {
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
    pub fn command15_done(&self) -> COMMAND15_DONE_R {
        COMMAND15_DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command15(&self) -> COMMAND15_R {
        COMMAND15_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command15(&mut self) -> COMMAND15_W {
        COMMAND15_W { w: self }
    }
}
