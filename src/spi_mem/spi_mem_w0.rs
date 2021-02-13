#[doc = "Reader of register SPI_MEM_W0"]
pub type R = crate::R<u32, super::SPI_MEM_W0>;
#[doc = "Writer for register SPI_MEM_W0"]
pub type W = crate::W<u32, super::SPI_MEM_W0>;
#[doc = "Register SPI_MEM_W0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_MEM_W0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_MEM_BUF0`"]
pub type SPI_MEM_BUF0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SPI_MEM_BUF0`"]
pub struct SPI_MEM_BUF0_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_BUF0_W<'a> {
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
    pub fn spi_mem_buf0(&self) -> SPI_MEM_BUF0_R {
        SPI_MEM_BUF0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn spi_mem_buf0(&mut self) -> SPI_MEM_BUF0_W {
        SPI_MEM_BUF0_W { w: self }
    }
}
