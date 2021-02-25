#[doc = "Reader of register EXT_WAKEUP_CONF"]
pub type R = crate::R<u32, super::EXT_WAKEUP_CONF>;
#[doc = "Writer for register EXT_WAKEUP_CONF"]
pub type W = crate::W<u32, super::EXT_WAKEUP_CONF>;
#[doc = "Register EXT_WAKEUP_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::EXT_WAKEUP_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO_WAKEUP_FILTER`"]
pub type GPIO_WAKEUP_FILTER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_WAKEUP_FILTER`"]
pub struct GPIO_WAKEUP_FILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_WAKEUP_FILTER_W<'a> {
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
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gpio_wakeup_filter(&self) -> GPIO_WAKEUP_FILTER_R {
        GPIO_WAKEUP_FILTER_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gpio_wakeup_filter(&mut self) -> GPIO_WAKEUP_FILTER_W {
        GPIO_WAKEUP_FILTER_W { w: self }
    }
}
