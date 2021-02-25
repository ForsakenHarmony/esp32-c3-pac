#[doc = "Reader of register RD_MAC_SPI_SYS_2"]
pub type R = crate::R<u32, super::RD_MAC_SPI_SYS_2>;
#[doc = "Reader of field `SPI_PAD_CONF_1`"]
pub type SPI_PAD_CONF_1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn spi_pad_conf_1(&self) -> SPI_PAD_CONF_1_R {
        SPI_PAD_CONF_1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
