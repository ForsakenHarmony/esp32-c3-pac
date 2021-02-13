#[doc = "Reader of register UHCI_ESC_CONF3"]
pub type R = crate::R<u32, super::UHCI_ESC_CONF3>;
#[doc = "Writer for register UHCI_ESC_CONF3"]
pub type W = crate::W<u32, super::UHCI_ESC_CONF3>;
#[doc = "Register UHCI_ESC_CONF3 `reset()`'s with value 0"]
impl crate::ResetValue for super::UHCI_ESC_CONF3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UHCI_ESC_SEQ2_CHAR1`"]
pub type UHCI_ESC_SEQ2_CHAR1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UHCI_ESC_SEQ2_CHAR1`"]
pub struct UHCI_ESC_SEQ2_CHAR1_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_ESC_SEQ2_CHAR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `UHCI_ESC_SEQ2_CHAR0`"]
pub type UHCI_ESC_SEQ2_CHAR0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UHCI_ESC_SEQ2_CHAR0`"]
pub struct UHCI_ESC_SEQ2_CHAR0_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_ESC_SEQ2_CHAR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `UHCI_ESC_SEQ2`"]
pub type UHCI_ESC_SEQ2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UHCI_ESC_SEQ2`"]
pub struct UHCI_ESC_SEQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_ESC_SEQ2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn uhci_esc_seq2_char1(&self) -> UHCI_ESC_SEQ2_CHAR1_R {
        UHCI_ESC_SEQ2_CHAR1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn uhci_esc_seq2_char0(&self) -> UHCI_ESC_SEQ2_CHAR0_R {
        UHCI_ESC_SEQ2_CHAR0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn uhci_esc_seq2(&self) -> UHCI_ESC_SEQ2_R {
        UHCI_ESC_SEQ2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn uhci_esc_seq2_char1(&mut self) -> UHCI_ESC_SEQ2_CHAR1_W {
        UHCI_ESC_SEQ2_CHAR1_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn uhci_esc_seq2_char0(&mut self) -> UHCI_ESC_SEQ2_CHAR0_W {
        UHCI_ESC_SEQ2_CHAR0_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn uhci_esc_seq2(&mut self) -> UHCI_ESC_SEQ2_W {
        UHCI_ESC_SEQ2_W { w: self }
    }
}
