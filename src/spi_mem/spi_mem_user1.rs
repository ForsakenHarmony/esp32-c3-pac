#[doc = "Reader of register SPI_MEM_USER1"]
pub type R = crate::R<u32, super::SPI_MEM_USER1>;
#[doc = "Writer for register SPI_MEM_USER1"]
pub type W = crate::W<u32, super::SPI_MEM_USER1>;
#[doc = "Register SPI_MEM_USER1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_MEM_USER1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_MEM_USR_ADDR_BITLEN`"]
pub type SPI_MEM_USR_ADDR_BITLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_MEM_USR_ADDR_BITLEN`"]
pub struct SPI_MEM_USR_ADDR_BITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_USR_ADDR_BITLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | (((value as u32) & 0x3f) << 26);
        self.w
    }
}
#[doc = "Reader of field `SPI_MEM_USR_DUMMY_CYCLELEN`"]
pub type SPI_MEM_USR_DUMMY_CYCLELEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_MEM_USR_DUMMY_CYCLELEN`"]
pub struct SPI_MEM_USR_DUMMY_CYCLELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_USR_DUMMY_CYCLELEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn spi_mem_usr_addr_bitlen(&self) -> SPI_MEM_USR_ADDR_BITLEN_R {
        SPI_MEM_USR_ADDR_BITLEN_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn spi_mem_usr_dummy_cyclelen(&self) -> SPI_MEM_USR_DUMMY_CYCLELEN_R {
        SPI_MEM_USR_DUMMY_CYCLELEN_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn spi_mem_usr_addr_bitlen(&mut self) -> SPI_MEM_USR_ADDR_BITLEN_W {
        SPI_MEM_USR_ADDR_BITLEN_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn spi_mem_usr_dummy_cyclelen(&mut self) -> SPI_MEM_USR_DUMMY_CYCLELEN_W {
        SPI_MEM_USR_DUMMY_CYCLELEN_W { w: self }
    }
}
