#[doc = "Reader of register RD_REPEAT_DATA0"]
pub type R = crate::R<u32, super::RD_REPEAT_DATA0>;
#[doc = "Reader of field `POWER_GLITCH_DSENSE`"]
pub type POWER_GLITCH_DSENSE_R = crate::R<u8, u8>;
#[doc = "Reader of field `POWERGLITCH_EN`"]
pub type POWERGLITCH_EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `BTLC_GPIO_ENABLE`"]
pub type BTLC_GPIO_ENABLE_R = crate::R<u8, u8>;
#[doc = "Reader of field `VDD_SPI_AS_GPIO`"]
pub type VDD_SPI_AS_GPIO_R = crate::R<bool, bool>;
#[doc = "Reader of field `USB_EXCHG_PINS`"]
pub type USB_EXCHG_PINS_R = crate::R<bool, bool>;
#[doc = "Reader of field `USB_DREFL`"]
pub type USB_DREFL_R = crate::R<u8, u8>;
#[doc = "Reader of field `USB_DREFH`"]
pub type USB_DREFH_R = crate::R<u8, u8>;
#[doc = "Reader of field `DIS_DOWNLOAD_MANUAL_ENCRYPT`"]
pub type DIS_DOWNLOAD_MANUAL_ENCRYPT_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIS_PAD_JTAG`"]
pub type DIS_PAD_JTAG_R = crate::R<bool, bool>;
#[doc = "Reader of field `SOFT_DIS_JTAG`"]
pub type SOFT_DIS_JTAG_R = crate::R<u8, u8>;
#[doc = "Reader of field `JTAG_SEL_ENABLE`"]
pub type JTAG_SEL_ENABLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIS_TWAI`"]
pub type DIS_TWAI_R = crate::R<bool, bool>;
#[doc = "Reader of field `RPT4_RESERVED6`"]
pub type RPT4_RESERVED6_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIS_FORCE_DOWNLOAD`"]
pub type DIS_FORCE_DOWNLOAD_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIS_USB_DEVICE`"]
pub type DIS_USB_DEVICE_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIS_DOWNLOAD_ICACHE`"]
pub type DIS_DOWNLOAD_ICACHE_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIS_USB_JTAG`"]
pub type DIS_USB_JTAG_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIS_ICACHE`"]
pub type DIS_ICACHE_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIS_RTC_RAM_BOOT`"]
pub type DIS_RTC_RAM_BOOT_R = crate::R<bool, bool>;
#[doc = "Reader of field `RD_DIS`"]
pub type RD_DIS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn power_glitch_dsense(&self) -> POWER_GLITCH_DSENSE_R {
        POWER_GLITCH_DSENSE_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn powerglitch_en(&self) -> POWERGLITCH_EN_R {
        POWERGLITCH_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn btlc_gpio_enable(&self) -> BTLC_GPIO_ENABLE_R {
        BTLC_GPIO_ENABLE_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn vdd_spi_as_gpio(&self) -> VDD_SPI_AS_GPIO_R {
        VDD_SPI_AS_GPIO_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn usb_exchg_pins(&self) -> USB_EXCHG_PINS_R {
        USB_EXCHG_PINS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 23:24"]
    #[inline(always)]
    pub fn usb_drefl(&self) -> USB_DREFL_R {
        USB_DREFL_R::new(((self.bits >> 23) & 0x03) as u8)
    }
    #[doc = "Bits 21:22"]
    #[inline(always)]
    pub fn usb_drefh(&self) -> USB_DREFH_R {
        USB_DREFH_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn dis_download_manual_encrypt(&self) -> DIS_DOWNLOAD_MANUAL_ENCRYPT_R {
        DIS_DOWNLOAD_MANUAL_ENCRYPT_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn dis_pad_jtag(&self) -> DIS_PAD_JTAG_R {
        DIS_PAD_JTAG_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn soft_dis_jtag(&self) -> SOFT_DIS_JTAG_R {
        SOFT_DIS_JTAG_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn jtag_sel_enable(&self) -> JTAG_SEL_ENABLE_R {
        JTAG_SEL_ENABLE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dis_twai(&self) -> DIS_TWAI_R {
        DIS_TWAI_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rpt4_reserved6(&self) -> RPT4_RESERVED6_R {
        RPT4_RESERVED6_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dis_force_download(&self) -> DIS_FORCE_DOWNLOAD_R {
        DIS_FORCE_DOWNLOAD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dis_usb_device(&self) -> DIS_USB_DEVICE_R {
        DIS_USB_DEVICE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dis_download_icache(&self) -> DIS_DOWNLOAD_ICACHE_R {
        DIS_DOWNLOAD_ICACHE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dis_usb_jtag(&self) -> DIS_USB_JTAG_R {
        DIS_USB_JTAG_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dis_icache(&self) -> DIS_ICACHE_R {
        DIS_ICACHE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dis_rtc_ram_boot(&self) -> DIS_RTC_RAM_BOOT_R {
        DIS_RTC_RAM_BOOT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn rd_dis(&self) -> RD_DIS_R {
        RD_DIS_R::new((self.bits & 0x7f) as u8)
    }
}
