#[doc = "Reader of register PIN%s"]
pub type R = crate::R<u32, super::PIN>;
#[doc = "Writer for register PIN%s"]
pub type W = crate::W<u32, super::PIN>;
#[doc = "Register PIN%s `reset()`'s with value 0"]
impl crate::ResetValue for super::PIN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INT_ENA`"]
pub type INT_ENA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT_ENA`"]
pub struct INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_ENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 13)) | (((value as u32) & 0x1f) << 13);
        self.w
    }
}
#[doc = "Reader of field `CONFIG`"]
pub type CONFIG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CONFIG`"]
pub struct CONFIG_W<'a> {
    w: &'a mut W,
}
impl<'a> CONFIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Reader of field `WAKEUP_ENABLE`"]
pub type WAKEUP_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKEUP_ENABLE`"]
pub struct WAKEUP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_ENABLE_W<'a> {
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
#[doc = "Reader of field `INT_TYPE`"]
pub type INT_TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT_TYPE`"]
pub struct INT_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 7)) | (((value as u32) & 0x07) << 7);
        self.w
    }
}
#[doc = "Reader of field `SYNC1_BYPASS`"]
pub type SYNC1_BYPASS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYNC1_BYPASS`"]
pub struct SYNC1_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC1_BYPASS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `PAD_DRIVER`"]
pub type PAD_DRIVER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD_DRIVER`"]
pub struct PAD_DRIVER_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_DRIVER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `SYNC2_BYPASS`"]
pub type SYNC2_BYPASS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYNC2_BYPASS`"]
pub struct SYNC2_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC2_BYPASS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 13:17"]
    #[inline(always)]
    pub fn int_ena(&self) -> INT_ENA_R {
        INT_ENA_R::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn config(&self) -> CONFIG_R {
        CONFIG_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn wakeup_enable(&self) -> WAKEUP_ENABLE_R {
        WAKEUP_ENABLE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn int_type(&self) -> INT_TYPE_R {
        INT_TYPE_R::new(((self.bits >> 7) & 0x07) as u8)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn sync1_bypass(&self) -> SYNC1_BYPASS_R {
        SYNC1_BYPASS_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pad_driver(&self) -> PAD_DRIVER_R {
        PAD_DRIVER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sync2_bypass(&self) -> SYNC2_BYPASS_R {
        SYNC2_BYPASS_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 13:17"]
    #[inline(always)]
    pub fn int_ena(&mut self) -> INT_ENA_W {
        INT_ENA_W { w: self }
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn config(&mut self) -> CONFIG_W {
        CONFIG_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn wakeup_enable(&mut self) -> WAKEUP_ENABLE_W {
        WAKEUP_ENABLE_W { w: self }
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn int_type(&mut self) -> INT_TYPE_W {
        INT_TYPE_W { w: self }
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn sync1_bypass(&mut self) -> SYNC1_BYPASS_W {
        SYNC1_BYPASS_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pad_driver(&mut self) -> PAD_DRIVER_W {
        PAD_DRIVER_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sync2_bypass(&mut self) -> SYNC2_BYPASS_W {
        SYNC2_BYPASS_W { w: self }
    }
}
