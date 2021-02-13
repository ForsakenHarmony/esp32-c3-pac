#[doc = "Reader of register GPIO_SDIO_SELECT"]
pub type R = crate::R<u32, super::GPIO_SDIO_SELECT>;
#[doc = "Writer for register GPIO_SDIO_SELECT"]
pub type W = crate::W<u32, super::GPIO_SDIO_SELECT>;
#[doc = "Register GPIO_SDIO_SELECT `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_SDIO_SELECT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO_SDIO_SEL`"]
pub type GPIO_SDIO_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPIO_SDIO_SEL`"]
pub struct GPIO_SDIO_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_SDIO_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn gpio_sdio_sel(&self) -> GPIO_SDIO_SEL_R {
        GPIO_SDIO_SEL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn gpio_sdio_sel(&mut self) -> GPIO_SDIO_SEL_W {
        GPIO_SDIO_SEL_W { w: self }
    }
}
