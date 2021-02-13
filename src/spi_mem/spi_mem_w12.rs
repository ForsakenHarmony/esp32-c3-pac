#[doc = "Reader of register SPI_MEM_W12"]
pub type R = crate::R<u32, super::SPI_MEM_W12>;
#[doc = "Writer for register SPI_MEM_W12"]
pub type W = crate::W<u32, super::SPI_MEM_W12>;
#[doc = "Register SPI_MEM_W12 `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_MEM_W12 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_MEM_BUF12`"]
pub type SPI_MEM_BUF12_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SPI_MEM_BUF12`"]
pub struct SPI_MEM_BUF12_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_BUF12_W<'a> {
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
    pub fn spi_mem_buf12(&self) -> SPI_MEM_BUF12_R {
        SPI_MEM_BUF12_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn spi_mem_buf12(&mut self) -> SPI_MEM_BUF12_W {
        SPI_MEM_BUF12_W { w: self }
    }
}
