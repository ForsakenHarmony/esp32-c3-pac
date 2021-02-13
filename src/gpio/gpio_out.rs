#[doc = "Reader of register GPIO_OUT"]
pub type R = crate::R<u32, super::GPIO_OUT>;
#[doc = "Writer for register GPIO_OUT"]
pub type W = crate::W<u32, super::GPIO_OUT>;
#[doc = "Register GPIO_OUT `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_OUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO_OUT_DATA`"]
pub type GPIO_OUT_DATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `GPIO_OUT_DATA`"]
pub struct GPIO_OUT_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_OUT_DATA_W<'a> {
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
    pub fn gpio_out_data(&self) -> GPIO_OUT_DATA_R {
        GPIO_OUT_DATA_R::new((self.bits & 0x03ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn gpio_out_data(&mut self) -> GPIO_OUT_DATA_W {
        GPIO_OUT_DATA_W { w: self }
    }
}
