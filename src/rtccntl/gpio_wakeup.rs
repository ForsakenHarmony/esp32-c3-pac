#[doc = "Reader of register GPIO_WAKEUP"]
pub type R = crate::R<u32, super::GPIO_WAKEUP>;
#[doc = "Writer for register GPIO_WAKEUP"]
pub type W = crate::W<u32, super::GPIO_WAKEUP>;
#[doc = "Register GPIO_WAKEUP `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_WAKEUP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO_PIN0_WAKEUP_ENABLE`"]
pub type GPIO_PIN0_WAKEUP_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_PIN0_WAKEUP_ENABLE`"]
pub struct GPIO_PIN0_WAKEUP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PIN0_WAKEUP_ENABLE_W<'a> {
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
#[doc = "Reader of field `GPIO_PIN1_WAKEUP_ENABLE`"]
pub type GPIO_PIN1_WAKEUP_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_PIN1_WAKEUP_ENABLE`"]
pub struct GPIO_PIN1_WAKEUP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PIN1_WAKEUP_ENABLE_W<'a> {
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
#[doc = "Reader of field `GPIO_PIN2_WAKEUP_ENABLE`"]
pub type GPIO_PIN2_WAKEUP_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_PIN2_WAKEUP_ENABLE`"]
pub struct GPIO_PIN2_WAKEUP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PIN2_WAKEUP_ENABLE_W<'a> {
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
#[doc = "Reader of field `GPIO_PIN3_WAKEUP_ENABLE`"]
pub type GPIO_PIN3_WAKEUP_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_PIN3_WAKEUP_ENABLE`"]
pub struct GPIO_PIN3_WAKEUP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PIN3_WAKEUP_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `GPIO_PIN4_WAKEUP_ENABLE`"]
pub type GPIO_PIN4_WAKEUP_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_PIN4_WAKEUP_ENABLE`"]
pub struct GPIO_PIN4_WAKEUP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PIN4_WAKEUP_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `GPIO_PIN5_WAKEUP_ENABLE`"]
pub type GPIO_PIN5_WAKEUP_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_PIN5_WAKEUP_ENABLE`"]
pub struct GPIO_PIN5_WAKEUP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PIN5_WAKEUP_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `GPIO_PIN0_INT_TYPE`"]
pub type GPIO_PIN0_INT_TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPIO_PIN0_INT_TYPE`"]
pub struct GPIO_PIN0_INT_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PIN0_INT_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 23)) | (((value as u32) & 0x07) << 23);
        self.w
    }
}
#[doc = "Reader of field `GPIO_PIN1_INT_TYPE`"]
pub type GPIO_PIN1_INT_TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPIO_PIN1_INT_TYPE`"]
pub struct GPIO_PIN1_INT_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PIN1_INT_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `GPIO_PIN2_INT_TYPE`"]
pub type GPIO_PIN2_INT_TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPIO_PIN2_INT_TYPE`"]
pub struct GPIO_PIN2_INT_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PIN2_INT_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | (((value as u32) & 0x07) << 17);
        self.w
    }
}
#[doc = "Reader of field `GPIO_PIN3_INT_TYPE`"]
pub type GPIO_PIN3_INT_TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPIO_PIN3_INT_TYPE`"]
pub struct GPIO_PIN3_INT_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PIN3_INT_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 14)) | (((value as u32) & 0x07) << 14);
        self.w
    }
}
#[doc = "Reader of field `GPIO_PIN4_INT_TYPE`"]
pub type GPIO_PIN4_INT_TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPIO_PIN4_INT_TYPE`"]
pub struct GPIO_PIN4_INT_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PIN4_INT_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Reader of field `GPIO_PIN5_INT_TYPE`"]
pub type GPIO_PIN5_INT_TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPIO_PIN5_INT_TYPE`"]
pub struct GPIO_PIN5_INT_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PIN5_INT_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `GPIO_PIN_CLK_GATE`"]
pub type GPIO_PIN_CLK_GATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_PIN_CLK_GATE`"]
pub struct GPIO_PIN_CLK_GATE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PIN_CLK_GATE_W<'a> {
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
#[doc = "Reader of field `GPIO_WAKEUP_STATUS_CLR`"]
pub type GPIO_WAKEUP_STATUS_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_WAKEUP_STATUS_CLR`"]
pub struct GPIO_WAKEUP_STATUS_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_WAKEUP_STATUS_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `GPIO_WAKEUP_STATUS`"]
pub type GPIO_WAKEUP_STATUS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gpio_pin0_wakeup_enable(&self) -> GPIO_PIN0_WAKEUP_ENABLE_R {
        GPIO_PIN0_WAKEUP_ENABLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn gpio_pin1_wakeup_enable(&self) -> GPIO_PIN1_WAKEUP_ENABLE_R {
        GPIO_PIN1_WAKEUP_ENABLE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn gpio_pin2_wakeup_enable(&self) -> GPIO_PIN2_WAKEUP_ENABLE_R {
        GPIO_PIN2_WAKEUP_ENABLE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn gpio_pin3_wakeup_enable(&self) -> GPIO_PIN3_WAKEUP_ENABLE_R {
        GPIO_PIN3_WAKEUP_ENABLE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn gpio_pin4_wakeup_enable(&self) -> GPIO_PIN4_WAKEUP_ENABLE_R {
        GPIO_PIN4_WAKEUP_ENABLE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn gpio_pin5_wakeup_enable(&self) -> GPIO_PIN5_WAKEUP_ENABLE_R {
        GPIO_PIN5_WAKEUP_ENABLE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 23:25"]
    #[inline(always)]
    pub fn gpio_pin0_int_type(&self) -> GPIO_PIN0_INT_TYPE_R {
        GPIO_PIN0_INT_TYPE_R::new(((self.bits >> 23) & 0x07) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn gpio_pin1_int_type(&self) -> GPIO_PIN1_INT_TYPE_R {
        GPIO_PIN1_INT_TYPE_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn gpio_pin2_int_type(&self) -> GPIO_PIN2_INT_TYPE_R {
        GPIO_PIN2_INT_TYPE_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 14:16"]
    #[inline(always)]
    pub fn gpio_pin3_int_type(&self) -> GPIO_PIN3_INT_TYPE_R {
        GPIO_PIN3_INT_TYPE_R::new(((self.bits >> 14) & 0x07) as u8)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn gpio_pin4_int_type(&self) -> GPIO_PIN4_INT_TYPE_R {
        GPIO_PIN4_INT_TYPE_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn gpio_pin5_int_type(&self) -> GPIO_PIN5_INT_TYPE_R {
        GPIO_PIN5_INT_TYPE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpio_pin_clk_gate(&self) -> GPIO_PIN_CLK_GATE_R {
        GPIO_PIN_CLK_GATE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpio_wakeup_status_clr(&self) -> GPIO_WAKEUP_STATUS_CLR_R {
        GPIO_WAKEUP_STATUS_CLR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn gpio_wakeup_status(&self) -> GPIO_WAKEUP_STATUS_R {
        GPIO_WAKEUP_STATUS_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gpio_pin0_wakeup_enable(&mut self) -> GPIO_PIN0_WAKEUP_ENABLE_W {
        GPIO_PIN0_WAKEUP_ENABLE_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn gpio_pin1_wakeup_enable(&mut self) -> GPIO_PIN1_WAKEUP_ENABLE_W {
        GPIO_PIN1_WAKEUP_ENABLE_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn gpio_pin2_wakeup_enable(&mut self) -> GPIO_PIN2_WAKEUP_ENABLE_W {
        GPIO_PIN2_WAKEUP_ENABLE_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn gpio_pin3_wakeup_enable(&mut self) -> GPIO_PIN3_WAKEUP_ENABLE_W {
        GPIO_PIN3_WAKEUP_ENABLE_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn gpio_pin4_wakeup_enable(&mut self) -> GPIO_PIN4_WAKEUP_ENABLE_W {
        GPIO_PIN4_WAKEUP_ENABLE_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn gpio_pin5_wakeup_enable(&mut self) -> GPIO_PIN5_WAKEUP_ENABLE_W {
        GPIO_PIN5_WAKEUP_ENABLE_W { w: self }
    }
    #[doc = "Bits 23:25"]
    #[inline(always)]
    pub fn gpio_pin0_int_type(&mut self) -> GPIO_PIN0_INT_TYPE_W {
        GPIO_PIN0_INT_TYPE_W { w: self }
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn gpio_pin1_int_type(&mut self) -> GPIO_PIN1_INT_TYPE_W {
        GPIO_PIN1_INT_TYPE_W { w: self }
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn gpio_pin2_int_type(&mut self) -> GPIO_PIN2_INT_TYPE_W {
        GPIO_PIN2_INT_TYPE_W { w: self }
    }
    #[doc = "Bits 14:16"]
    #[inline(always)]
    pub fn gpio_pin3_int_type(&mut self) -> GPIO_PIN3_INT_TYPE_W {
        GPIO_PIN3_INT_TYPE_W { w: self }
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn gpio_pin4_int_type(&mut self) -> GPIO_PIN4_INT_TYPE_W {
        GPIO_PIN4_INT_TYPE_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn gpio_pin5_int_type(&mut self) -> GPIO_PIN5_INT_TYPE_W {
        GPIO_PIN5_INT_TYPE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpio_pin_clk_gate(&mut self) -> GPIO_PIN_CLK_GATE_W {
        GPIO_PIN_CLK_GATE_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpio_wakeup_status_clr(&mut self) -> GPIO_WAKEUP_STATUS_CLR_W {
        GPIO_WAKEUP_STATUS_CLR_W { w: self }
    }
}
