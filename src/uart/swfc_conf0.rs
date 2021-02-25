#[doc = "Reader of register SWFC_CONF0"]
pub type R = crate::R<u32, super::SWFC_CONF0>;
#[doc = "Writer for register SWFC_CONF0"]
pub type W = crate::W<u32, super::SWFC_CONF0>;
#[doc = "Register SWFC_CONF0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SWFC_CONF0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XOFF_CHAR`"]
pub type XOFF_CHAR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XOFF_CHAR`"]
pub struct XOFF_CHAR_W<'a> {
    w: &'a mut W,
}
impl<'a> XOFF_CHAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 9)) | (((value as u32) & 0xff) << 9);
        self.w
    }
}
#[doc = "Reader of field `XOFF_THRESHOLD`"]
pub type XOFF_THRESHOLD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `XOFF_THRESHOLD`"]
pub struct XOFF_THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> XOFF_THRESHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 9:16"]
    #[inline(always)]
    pub fn xoff_char(&self) -> XOFF_CHAR_R {
        XOFF_CHAR_R::new(((self.bits >> 9) & 0xff) as u8)
    }
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn xoff_threshold(&self) -> XOFF_THRESHOLD_R {
        XOFF_THRESHOLD_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 9:16"]
    #[inline(always)]
    pub fn xoff_char(&mut self) -> XOFF_CHAR_W {
        XOFF_CHAR_W { w: self }
    }
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn xoff_threshold(&mut self) -> XOFF_THRESHOLD_W {
        XOFF_THRESHOLD_W { w: self }
    }
}
