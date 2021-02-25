#[doc = "Reader of register RD_MAC_SPI_SYS_1"]
pub type R = crate::R<u32, super::RD_MAC_SPI_SYS_1>;
#[doc = "Reader of field `SPI_PAD_CONF_0`"]
pub type SPI_PAD_CONF_0_R = crate::R<u16, u16>;
#[doc = "Reader of field `MAC_1`"]
pub type MAC_1_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn spi_pad_conf_0(&self) -> SPI_PAD_CONF_0_R {
        SPI_PAD_CONF_0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mac_1(&self) -> MAC_1_R {
        MAC_1_R::new((self.bits & 0xffff) as u16)
    }
}
