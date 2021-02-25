#[doc = "Reader of register COMD0"]
pub type R = crate::R<u32, super::COMD0>;
#[doc = "Writer for register COMD0"]
pub type W = crate::W<u32, super::COMD0>;
#[doc = "Register COMD0 `reset()`'s with value 0"]
impl crate::ResetValue for super::COMD0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMMAND0_DONE`"]
pub type COMMAND0_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMMAND0_DONE`"]
pub struct COMMAND0_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND0_DONE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `COMMAND0`"]
pub type COMMAND0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COMMAND0`"]
pub struct COMMAND0_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND0_W<'a> {
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
    pub fn command0_done(&self) -> COMMAND0_DONE_R {
        COMMAND0_DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command0(&self) -> COMMAND0_R {
        COMMAND0_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn command0_done(&mut self) -> COMMAND0_DONE_W {
        COMMAND0_DONE_W { w: self }
    }
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command0(&mut self) -> COMMAND0_W {
        COMMAND0_W { w: self }
    }
}
