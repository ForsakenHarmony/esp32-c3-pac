#[doc = "Reader of register GPIO_STATUS"]
pub type R = crate::R<u32, super::GPIO_STATUS>;
#[doc = "Writer for register GPIO_STATUS"]
pub type W = crate::W<u32, super::GPIO_STATUS>;
#[doc = "Register GPIO_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO_STATUS_INT`"]
pub type GPIO_STATUS_INT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `GPIO_STATUS_INT`"]
pub struct GPIO_STATUS_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_STATUS_INT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff_ffff) | ((value as u32) & 0x03ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn gpio_status_int(&self) -> GPIO_STATUS_INT_R {
        GPIO_STATUS_INT_R::new((self.bits & 0x03ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn gpio_status_int(&mut self) -> GPIO_STATUS_INT_W {
        GPIO_STATUS_INT_W { w: self }
    }
}
