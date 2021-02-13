#[doc = "Reader of register GPIO_PIN17"]
pub type R = crate::R<u32, super::GPIO_PIN17>;
#[doc = "Writer for register GPIO_PIN17"]
pub type W = crate::W<u32, super::GPIO_PIN17>;
#[doc = "Register GPIO_PIN17 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_PIN17 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO_PIN17_INT_ENA`"]
pub type GPIO_PIN17_INT_ENA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPIO_PIN17_INT_ENA`"]
pub struct GPIO_PIN17_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PIN17_INT_ENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 13)) | (((value as u32) & 0x1f) << 13);
        self.w
    }
}
#[doc = "Reader of field `GPIO_PIN17_CONFIG`"]
pub type GPIO_PIN17_CONFIG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPIO_PIN17_CONFIG`"]
pub struct GPIO_PIN17_CONFIG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PIN17_CONFIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Reader of field `GPIO_PIN17_WAKEUP_ENABLE`"]
pub type GPIO_PIN17_WAKEUP_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_PIN17_WAKEUP_ENABLE`"]
pub struct GPIO_PIN17_WAKEUP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PIN17_WAKEUP_ENABLE_W<'a> {
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
#[doc = "Reader of field `GPIO_PIN17_INT_TYPE`"]
pub type GPIO_PIN17_INT_TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPIO_PIN17_INT_TYPE`"]
pub struct GPIO_PIN17_INT_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PIN17_INT_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 7)) | (((value as u32) & 0x07) << 7);
        self.w
    }
}
#[doc = "Reader of field `GPIO_PIN17_SYNC1_BYPASS`"]
pub type GPIO_PIN17_SYNC1_BYPASS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPIO_PIN17_SYNC1_BYPASS`"]
pub struct GPIO_PIN17_SYNC1_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PIN17_SYNC1_BYPASS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `GPIO_PIN17_PAD_DRIVER`"]
pub type GPIO_PIN17_PAD_DRIVER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_PIN17_PAD_DRIVER`"]
pub struct GPIO_PIN17_PAD_DRIVER_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PIN17_PAD_DRIVER_W<'a> {
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
#[doc = "Reader of field `GPIO_PIN17_SYNC2_BYPASS`"]
pub type GPIO_PIN17_SYNC2_BYPASS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPIO_PIN17_SYNC2_BYPASS`"]
pub struct GPIO_PIN17_SYNC2_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PIN17_SYNC2_BYPASS_W<'a> {
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
    pub fn gpio_pin17_int_ena(&self) -> GPIO_PIN17_INT_ENA_R {
        GPIO_PIN17_INT_ENA_R::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn gpio_pin17_config(&self) -> GPIO_PIN17_CONFIG_R {
        GPIO_PIN17_CONFIG_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gpio_pin17_wakeup_enable(&self) -> GPIO_PIN17_WAKEUP_ENABLE_R {
        GPIO_PIN17_WAKEUP_ENABLE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn gpio_pin17_int_type(&self) -> GPIO_PIN17_INT_TYPE_R {
        GPIO_PIN17_INT_TYPE_R::new(((self.bits >> 7) & 0x07) as u8)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn gpio_pin17_sync1_bypass(&self) -> GPIO_PIN17_SYNC1_BYPASS_R {
        GPIO_PIN17_SYNC1_BYPASS_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpio_pin17_pad_driver(&self) -> GPIO_PIN17_PAD_DRIVER_R {
        GPIO_PIN17_PAD_DRIVER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn gpio_pin17_sync2_bypass(&self) -> GPIO_PIN17_SYNC2_BYPASS_R {
        GPIO_PIN17_SYNC2_BYPASS_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 13:17"]
    #[inline(always)]
    pub fn gpio_pin17_int_ena(&mut self) -> GPIO_PIN17_INT_ENA_W {
        GPIO_PIN17_INT_ENA_W { w: self }
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn gpio_pin17_config(&mut self) -> GPIO_PIN17_CONFIG_W {
        GPIO_PIN17_CONFIG_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gpio_pin17_wakeup_enable(&mut self) -> GPIO_PIN17_WAKEUP_ENABLE_W {
        GPIO_PIN17_WAKEUP_ENABLE_W { w: self }
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn gpio_pin17_int_type(&mut self) -> GPIO_PIN17_INT_TYPE_W {
        GPIO_PIN17_INT_TYPE_W { w: self }
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn gpio_pin17_sync1_bypass(&mut self) -> GPIO_PIN17_SYNC1_BYPASS_W {
        GPIO_PIN17_SYNC1_BYPASS_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpio_pin17_pad_driver(&mut self) -> GPIO_PIN17_PAD_DRIVER_W {
        GPIO_PIN17_PAD_DRIVER_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn gpio_pin17_sync2_bypass(&mut self) -> GPIO_PIN17_SYNC2_BYPASS_W {
        GPIO_PIN17_SYNC2_BYPASS_W { w: self }
    }
}
