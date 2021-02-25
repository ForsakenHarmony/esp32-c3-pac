#[doc = "Reader of register RD_REPEAT_ERR0"]
pub type R = crate::R<u32, super::RD_REPEAT_ERR0>;
#[doc = "Reader of field `POWER_GLITCH_DSENSE_ERR`"]
pub type POWER_GLITCH_DSENSE_ERR_R = crate::R<u8, u8>;
#[doc = "Reader of field `POWERGLITCH_EN_ERR`"]
pub type POWERGLITCH_EN_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `BTLC_GPIO_ENABLE_ERR`"]
pub type BTLC_GPIO_ENABLE_ERR_R = crate::R<u8, u8>;
#[doc = "Reader of field `VDD_SPI_AS_GPIO_ERR`"]
pub type VDD_SPI_AS_GPIO_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `USB_EXCHG_PINS_ERR`"]
pub type USB_EXCHG_PINS_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `USB_DREFL_ERR`"]
pub type USB_DREFL_ERR_R = crate::R<u8, u8>;
#[doc = "Reader of field `USB_DREFH_ERR`"]
pub type USB_DREFH_ERR_R = crate::R<u8, u8>;
#[doc = "Reader of field `DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR`"]
pub type DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIS_PAD_JTAG_ERR`"]
pub type DIS_PAD_JTAG_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `SOFT_DIS_JTAG_ERR`"]
pub type SOFT_DIS_JTAG_ERR_R = crate::R<u8, u8>;
#[doc = "Reader of field `JTAG_SEL_ENABLE_ERR`"]
pub type JTAG_SEL_ENABLE_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIS_TWAI_ERR`"]
pub type DIS_TWAI_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `RPT4_RESERVED6_ERR`"]
pub type RPT4_RESERVED6_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIS_FORCE_DOWNLOAD_ERR`"]
pub type DIS_FORCE_DOWNLOAD_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIS_USB_DEVICE_ERR`"]
pub type DIS_USB_DEVICE_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIS_DOWNLOAD_ICACHE`"]
pub type DIS_DOWNLOAD_ICACHE_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIS_USB_JTAG_ERR`"]
pub type DIS_USB_JTAG_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIS_ICACHE_ERR`"]
pub type DIS_ICACHE_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIS_RTC_RAM_BOOT_ERR`"]
pub type DIS_RTC_RAM_BOOT_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `RD_DIS_ERR`"]
pub type RD_DIS_ERR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn power_glitch_dsense_err(&self) -> POWER_GLITCH_DSENSE_ERR_R {
        POWER_GLITCH_DSENSE_ERR_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn powerglitch_en_err(&self) -> POWERGLITCH_EN_ERR_R {
        POWERGLITCH_EN_ERR_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn btlc_gpio_enable_err(&self) -> BTLC_GPIO_ENABLE_ERR_R {
        BTLC_GPIO_ENABLE_ERR_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn vdd_spi_as_gpio_err(&self) -> VDD_SPI_AS_GPIO_ERR_R {
        VDD_SPI_AS_GPIO_ERR_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn usb_exchg_pins_err(&self) -> USB_EXCHG_PINS_ERR_R {
        USB_EXCHG_PINS_ERR_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 23:24"]
    #[inline(always)]
    pub fn usb_drefl_err(&self) -> USB_DREFL_ERR_R {
        USB_DREFL_ERR_R::new(((self.bits >> 23) & 0x03) as u8)
    }
    #[doc = "Bits 21:22"]
    #[inline(always)]
    pub fn usb_drefh_err(&self) -> USB_DREFH_ERR_R {
        USB_DREFH_ERR_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn dis_download_manual_encrypt_err(&self) -> DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR_R {
        DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn dis_pad_jtag_err(&self) -> DIS_PAD_JTAG_ERR_R {
        DIS_PAD_JTAG_ERR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn soft_dis_jtag_err(&self) -> SOFT_DIS_JTAG_ERR_R {
        SOFT_DIS_JTAG_ERR_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn jtag_sel_enable_err(&self) -> JTAG_SEL_ENABLE_ERR_R {
        JTAG_SEL_ENABLE_ERR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dis_twai_err(&self) -> DIS_TWAI_ERR_R {
        DIS_TWAI_ERR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rpt4_reserved6_err(&self) -> RPT4_RESERVED6_ERR_R {
        RPT4_RESERVED6_ERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dis_force_download_err(&self) -> DIS_FORCE_DOWNLOAD_ERR_R {
        DIS_FORCE_DOWNLOAD_ERR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dis_usb_device_err(&self) -> DIS_USB_DEVICE_ERR_R {
        DIS_USB_DEVICE_ERR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dis_download_icache(&self) -> DIS_DOWNLOAD_ICACHE_R {
        DIS_DOWNLOAD_ICACHE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dis_usb_jtag_err(&self) -> DIS_USB_JTAG_ERR_R {
        DIS_USB_JTAG_ERR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dis_icache_err(&self) -> DIS_ICACHE_ERR_R {
        DIS_ICACHE_ERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dis_rtc_ram_boot_err(&self) -> DIS_RTC_RAM_BOOT_ERR_R {
        DIS_RTC_RAM_BOOT_ERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn rd_dis_err(&self) -> RD_DIS_ERR_R {
        RD_DIS_ERR_R::new((self.bits & 0x7f) as u8)
    }
}
