#[doc = "Reader of register CMD3"]
pub type R = crate::R<u32, super::CMD3>;
#[doc = "Writer for register CMD3"]
pub type W = crate::W<u32, super::CMD3>;
#[doc = "Register CMD3 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMMAND3_DONE`"]
pub type COMMAND3_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `COMMAND3`"]
pub type COMMAND3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COMMAND3`"]
pub struct COMMAND3_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND3_W<'a> {
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
    pub fn command3_done(&self) -> COMMAND3_DONE_R {
        COMMAND3_DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command3(&self) -> COMMAND3_R {
        COMMAND3_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command3(&mut self) -> COMMAND3_W {
        COMMAND3_W { w: self }
    }
}
