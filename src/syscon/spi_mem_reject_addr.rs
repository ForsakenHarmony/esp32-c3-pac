#[doc = "Reader of register SPI_MEM_REJECT_ADDR"]
pub type R = crate::R<u32, super::SPI_MEM_REJECT_ADDR>;
#[doc = "Reader of field `SPI_MEM_REJECT_ADDR`"]
pub type SPI_MEM_REJECT_ADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn spi_mem_reject_addr(&self) -> SPI_MEM_REJECT_ADDR_R {
        SPI_MEM_REJECT_ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
