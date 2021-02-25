#[doc = "Reader of register RD_REPEAT_DATA3"]
pub type R = crate::R<u32, super::RD_REPEAT_DATA3>;
#[doc = "Reader of field `RPT4_RESERVED1`"]
pub type RPT4_RESERVED1_R = crate::R<u8, u8>;
#[doc = "Reader of field `SECURE_VERSION`"]
pub type SECURE_VERSION_R = crate::R<u16, u16>;
#[doc = "Reader of field `FORCE_SEND_RESUME`"]
pub type FORCE_SEND_RESUME_R = crate::R<bool, bool>;
#[doc = "Reader of field `FLASH_ECC_EN`"]
pub type FLASH_ECC_EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `FLASH_PAGE_SIZE`"]
pub type FLASH_PAGE_SIZE_R = crate::R<u8, u8>;
#[doc = "Reader of field `FLASH_TYPE`"]
pub type FLASH_TYPE_R = crate::R<bool, bool>;
#[doc = "Reader of field `PIN_POWER_SELECTION`"]
pub type PIN_POWER_SELECTION_R = crate::R<bool, bool>;
#[doc = "Reader of field `UART_PRINT_CONTROL`"]
pub type UART_PRINT_CONTROL_R = crate::R<u8, u8>;
#[doc = "Reader of field `ENABLE_SECURITY_DOWNLOAD`"]
pub type ENABLE_SECURITY_DOWNLOAD_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIS_USB_DOWNLOAD_MODE`"]
pub type DIS_USB_DOWNLOAD_MODE_R = crate::R<bool, bool>;
#[doc = "Reader of field `FLASH_ECC_MODE`"]
pub type FLASH_ECC_MODE_R = crate::R<bool, bool>;
#[doc = "Reader of field `UART_PRINT_CHANNEL`"]
pub type UART_PRINT_CHANNEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIS_LEGACY_SPI_BOOT`"]
pub type DIS_LEGACY_SPI_BOOT_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIS_DOWNLOAD_MODE`"]
pub type DIS_DOWNLOAD_MODE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn rpt4_reserved1(&self) -> RPT4_RESERVED1_R {
        RPT4_RESERVED1_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 14:29"]
    #[inline(always)]
    pub fn secure_version(&self) -> SECURE_VERSION_R {
        SECURE_VERSION_R::new(((self.bits >> 14) & 0xffff) as u16)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn force_send_resume(&self) -> FORCE_SEND_RESUME_R {
        FORCE_SEND_RESUME_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn flash_ecc_en(&self) -> FLASH_ECC_EN_R {
        FLASH_ECC_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn flash_page_size(&self) -> FLASH_PAGE_SIZE_R {
        FLASH_PAGE_SIZE_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn flash_type(&self) -> FLASH_TYPE_R {
        FLASH_TYPE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pin_power_selection(&self) -> PIN_POWER_SELECTION_R {
        PIN_POWER_SELECTION_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn uart_print_control(&self) -> UART_PRINT_CONTROL_R {
        UART_PRINT_CONTROL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn enable_security_download(&self) -> ENABLE_SECURITY_DOWNLOAD_R {
        ENABLE_SECURITY_DOWNLOAD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dis_usb_download_mode(&self) -> DIS_USB_DOWNLOAD_MODE_R {
        DIS_USB_DOWNLOAD_MODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn flash_ecc_mode(&self) -> FLASH_ECC_MODE_R {
        FLASH_ECC_MODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn uart_print_channel(&self) -> UART_PRINT_CHANNEL_R {
        UART_PRINT_CHANNEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dis_legacy_spi_boot(&self) -> DIS_LEGACY_SPI_BOOT_R {
        DIS_LEGACY_SPI_BOOT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dis_download_mode(&self) -> DIS_DOWNLOAD_MODE_R {
        DIS_DOWNLOAD_MODE_R::new((self.bits & 0x01) != 0)
    }
}
