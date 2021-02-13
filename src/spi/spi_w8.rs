#[doc = "Reader of register SPI_W8"]
pub type R = crate::R<u32, super::SPI_W8>;
#[doc = "Writer for register SPI_W8"]
pub type W = crate::W<u32, super::SPI_W8>;
#[doc = "Register SPI_W8 `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_W8 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_BUF8`"]
pub type SPI_BUF8_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SPI_BUF8`"]
pub struct SPI_BUF8_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_BUF8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn spi_buf8(&self) -> SPI_BUF8_R {
        SPI_BUF8_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn spi_buf8(&mut self) -> SPI_BUF8_W {
        SPI_BUF8_W { w: self }
    }
}
