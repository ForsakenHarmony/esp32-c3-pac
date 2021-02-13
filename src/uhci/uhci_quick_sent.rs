#[doc = "Reader of register UHCI_QUICK_SENT"]
pub type R = crate::R<u32, super::UHCI_QUICK_SENT>;
#[doc = "Writer for register UHCI_QUICK_SENT"]
pub type W = crate::W<u32, super::UHCI_QUICK_SENT>;
#[doc = "Register UHCI_QUICK_SENT `reset()`'s with value 0"]
impl crate::ResetValue for super::UHCI_QUICK_SENT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UHCI_ALWAYS_SEND_EN`"]
pub type UHCI_ALWAYS_SEND_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_ALWAYS_SEND_EN`"]
pub struct UHCI_ALWAYS_SEND_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_ALWAYS_SEND_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `UHCI_ALWAYS_SEND_NUM`"]
pub type UHCI_ALWAYS_SEND_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UHCI_ALWAYS_SEND_NUM`"]
pub struct UHCI_ALWAYS_SEND_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_ALWAYS_SEND_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `UHCI_SINGLE_SEND_EN`"]
pub type UHCI_SINGLE_SEND_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_SINGLE_SEND_EN`"]
pub struct UHCI_SINGLE_SEND_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_SINGLE_SEND_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `UHCI_SINGLE_SEND_NUM`"]
pub type UHCI_SINGLE_SEND_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UHCI_SINGLE_SEND_NUM`"]
pub struct UHCI_SINGLE_SEND_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_SINGLE_SEND_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn uhci_always_send_en(&self) -> UHCI_ALWAYS_SEND_EN_R {
        UHCI_ALWAYS_SEND_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn uhci_always_send_num(&self) -> UHCI_ALWAYS_SEND_NUM_R {
        UHCI_ALWAYS_SEND_NUM_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn uhci_single_send_en(&self) -> UHCI_SINGLE_SEND_EN_R {
        UHCI_SINGLE_SEND_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn uhci_single_send_num(&self) -> UHCI_SINGLE_SEND_NUM_R {
        UHCI_SINGLE_SEND_NUM_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn uhci_always_send_en(&mut self) -> UHCI_ALWAYS_SEND_EN_W {
        UHCI_ALWAYS_SEND_EN_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn uhci_always_send_num(&mut self) -> UHCI_ALWAYS_SEND_NUM_W {
        UHCI_ALWAYS_SEND_NUM_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn uhci_single_send_en(&mut self) -> UHCI_SINGLE_SEND_EN_W {
        UHCI_SINGLE_SEND_EN_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn uhci_single_send_num(&mut self) -> UHCI_SINGLE_SEND_NUM_W {
        UHCI_SINGLE_SEND_NUM_W { w: self }
    }
}
