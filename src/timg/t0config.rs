#[doc = "Reader of register T0CONFIG"]
pub type R = crate::R<u32, super::T0CONFIG>;
#[doc = "Writer for register T0CONFIG"]
pub type W = crate::W<u32, super::T0CONFIG>;
#[doc = "Register T0CONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::T0CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `T0_EN`"]
pub type T0_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T0_EN`"]
pub struct T0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_EN_W<'a> {
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
#[doc = "Reader of field `T0_INCREASE`"]
pub type T0_INCREASE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T0_INCREASE`"]
pub struct T0_INCREASE_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_INCREASE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `T0_AUTORELOAD`"]
pub type T0_AUTORELOAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T0_AUTORELOAD`"]
pub struct T0_AUTORELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_AUTORELOAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `T0_DIVIDER`"]
pub type T0_DIVIDER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `T0_DIVIDER`"]
pub struct T0_DIVIDER_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_DIVIDER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 13)) | (((value as u32) & 0xffff) << 13);
        self.w
    }
}
#[doc = "Write proxy for field `T0_DIVCNT_RST`"]
pub struct T0_DIVCNT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_DIVCNT_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `T0_ALARM_EN`"]
pub type T0_ALARM_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T0_ALARM_EN`"]
pub struct T0_ALARM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_ALARM_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `T0_USE_XTAL`"]
pub type T0_USE_XTAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T0_USE_XTAL`"]
pub struct T0_USE_XTAL_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_USE_XTAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn t0_en(&self) -> T0_EN_R {
        T0_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn t0_increase(&self) -> T0_INCREASE_R {
        T0_INCREASE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn t0_autoreload(&self) -> T0_AUTORELOAD_R {
        T0_AUTORELOAD_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 13:28"]
    #[inline(always)]
    pub fn t0_divider(&self) -> T0_DIVIDER_R {
        T0_DIVIDER_R::new(((self.bits >> 13) & 0xffff) as u16)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn t0_alarm_en(&self) -> T0_ALARM_EN_R {
        T0_ALARM_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn t0_use_xtal(&self) -> T0_USE_XTAL_R {
        T0_USE_XTAL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn t0_en(&mut self) -> T0_EN_W {
        T0_EN_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn t0_increase(&mut self) -> T0_INCREASE_W {
        T0_INCREASE_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn t0_autoreload(&mut self) -> T0_AUTORELOAD_W {
        T0_AUTORELOAD_W { w: self }
    }
    #[doc = "Bits 13:28"]
    #[inline(always)]
    pub fn t0_divider(&mut self) -> T0_DIVIDER_W {
        T0_DIVIDER_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn t0_divcnt_rst(&mut self) -> T0_DIVCNT_RST_W {
        T0_DIVCNT_RST_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn t0_alarm_en(&mut self) -> T0_ALARM_EN_W {
        T0_ALARM_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn t0_use_xtal(&mut self) -> T0_USE_XTAL_W {
        T0_USE_XTAL_W { w: self }
    }
}
