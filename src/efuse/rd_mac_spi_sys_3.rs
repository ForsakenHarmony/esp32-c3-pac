#[doc = "Reader of register RD_MAC_SPI_SYS_3"]
pub type R = crate::R<u32, super::RD_MAC_SPI_SYS_3>;
#[doc = "Reader of field `SYS_DATA_PART0_0`"]
pub type SYS_DATA_PART0_0_R = crate::R<u8, u8>;
#[doc = "Reader of field `PKG_VERSION`"]
pub type PKG_VERSION_R = crate::R<u8, u8>;
#[doc = "Reader of field `WAFER_VERSION`"]
pub type WAFER_VERSION_R = crate::R<u8, u8>;
#[doc = "Reader of field `SPI_PAD_CONF_2`"]
pub type SPI_PAD_CONF_2_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn sys_data_part0_0(&self) -> SYS_DATA_PART0_0_R {
        SYS_DATA_PART0_0_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn pkg_version(&self) -> PKG_VERSION_R {
        PKG_VERSION_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn wafer_version(&self) -> WAFER_VERSION_R {
        WAFER_VERSION_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 0:17"]
    #[inline(always)]
    pub fn spi_pad_conf_2(&self) -> SPI_PAD_CONF_2_R {
        SPI_PAD_CONF_2_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
