#[doc = "Reader of register PGM_DATA1"]
pub type R = crate::R<u32, super::PGM_DATA1>;
#[doc = "Writer for register PGM_DATA1"]
pub type W = crate::W<u32, super::PGM_DATA1>;
#[doc = "Register PGM_DATA1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PGM_DATA1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `POWER_GLITCH_DSENSE`"]
pub type POWER_GLITCH_DSENSE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `POWER_GLITCH_DSENSE`"]
pub struct POWER_GLITCH_DSENSE_W<'a> {
    w: &'a mut W,
}
impl<'a> POWER_GLITCH_DSENSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `POWERGLITCH_EN`"]
pub type POWERGLITCH_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POWERGLITCH_EN`"]
pub struct POWERGLITCH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> POWERGLITCH_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `BTLC_GPIO_ENABLE`"]
pub type BTLC_GPIO_ENABLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BTLC_GPIO_ENABLE`"]
pub struct BTLC_GPIO_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> BTLC_GPIO_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
        self.w
    }
}
#[doc = "Reader of field `VDD_SPI_AS_GPIO`"]
pub type VDD_SPI_AS_GPIO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VDD_SPI_AS_GPIO`"]
pub struct VDD_SPI_AS_GPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> VDD_SPI_AS_GPIO_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `USB_EXCHG_PINS`"]
pub type USB_EXCHG_PINS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB_EXCHG_PINS`"]
pub struct USB_EXCHG_PINS_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_EXCHG_PINS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `USB_DREFL`"]
pub type USB_DREFL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USB_DREFL`"]
pub struct USB_DREFL_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_DREFL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 23)) | (((value as u32) & 0x03) << 23);
        self.w
    }
}
#[doc = "Reader of field `USB_DREFH`"]
pub type USB_DREFH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USB_DREFH`"]
pub struct USB_DREFH_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_DREFH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "Reader of field `DIS_DOWNLOAD_MANUAL_ENCRYPT`"]
pub type DIS_DOWNLOAD_MANUAL_ENCRYPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_DOWNLOAD_MANUAL_ENCRYPT`"]
pub struct DIS_DOWNLOAD_MANUAL_ENCRYPT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_DOWNLOAD_MANUAL_ENCRYPT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `DIS_PAD_JTAG`"]
pub type DIS_PAD_JTAG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_PAD_JTAG`"]
pub struct DIS_PAD_JTAG_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_PAD_JTAG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `SOFT_DIS_JTAG`"]
pub type SOFT_DIS_JTAG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SOFT_DIS_JTAG`"]
pub struct SOFT_DIS_JTAG_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_DIS_JTAG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `JTAG_SEL_ENABLE`"]
pub type JTAG_SEL_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JTAG_SEL_ENABLE`"]
pub struct JTAG_SEL_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> JTAG_SEL_ENABLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `DIS_TWAI`"]
pub type DIS_TWAI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_TWAI`"]
pub struct DIS_TWAI_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_TWAI_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `RPT4_RESERVED6_ERR`"]
pub type RPT4_RESERVED6_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIS_FORCE_DOWNLOAD`"]
pub type DIS_FORCE_DOWNLOAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_FORCE_DOWNLOAD`"]
pub struct DIS_FORCE_DOWNLOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_FORCE_DOWNLOAD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `DIS_USB_DEVICE`"]
pub type DIS_USB_DEVICE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_USB_DEVICE`"]
pub struct DIS_USB_DEVICE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_USB_DEVICE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `DIS_DOWNLOAD_ICACHE`"]
pub type DIS_DOWNLOAD_ICACHE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_DOWNLOAD_ICACHE`"]
pub struct DIS_DOWNLOAD_ICACHE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_DOWNLOAD_ICACHE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `DIS_USB_JTAG`"]
pub type DIS_USB_JTAG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_USB_JTAG`"]
pub struct DIS_USB_JTAG_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_USB_JTAG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `DIS_ICACHE`"]
pub type DIS_ICACHE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_ICACHE`"]
pub struct DIS_ICACHE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_ICACHE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `DIS_RTC_RAM_BOOT`"]
pub type DIS_RTC_RAM_BOOT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_RTC_RAM_BOOT`"]
pub struct DIS_RTC_RAM_BOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_RTC_RAM_BOOT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `RD_DIS`"]
pub type RD_DIS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RD_DIS`"]
pub struct RD_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_DIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
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
    pub fn rpt4_reserved6_err(&self) -> RPT4_RESERVED6_ERR_R {
        RPT4_RESERVED6_ERR_R::new(((self.bits >> 13) & 0x01) != 0)
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
impl W {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn power_glitch_dsense(&mut self) -> POWER_GLITCH_DSENSE_W {
        POWER_GLITCH_DSENSE_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn powerglitch_en(&mut self) -> POWERGLITCH_EN_W {
        POWERGLITCH_EN_W { w: self }
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn btlc_gpio_enable(&mut self) -> BTLC_GPIO_ENABLE_W {
        BTLC_GPIO_ENABLE_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn vdd_spi_as_gpio(&mut self) -> VDD_SPI_AS_GPIO_W {
        VDD_SPI_AS_GPIO_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn usb_exchg_pins(&mut self) -> USB_EXCHG_PINS_W {
        USB_EXCHG_PINS_W { w: self }
    }
    #[doc = "Bits 23:24"]
    #[inline(always)]
    pub fn usb_drefl(&mut self) -> USB_DREFL_W {
        USB_DREFL_W { w: self }
    }
    #[doc = "Bits 21:22"]
    #[inline(always)]
    pub fn usb_drefh(&mut self) -> USB_DREFH_W {
        USB_DREFH_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn dis_download_manual_encrypt(&mut self) -> DIS_DOWNLOAD_MANUAL_ENCRYPT_W {
        DIS_DOWNLOAD_MANUAL_ENCRYPT_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn dis_pad_jtag(&mut self) -> DIS_PAD_JTAG_W {
        DIS_PAD_JTAG_W { w: self }
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn soft_dis_jtag(&mut self) -> SOFT_DIS_JTAG_W {
        SOFT_DIS_JTAG_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn jtag_sel_enable(&mut self) -> JTAG_SEL_ENABLE_W {
        JTAG_SEL_ENABLE_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dis_twai(&mut self) -> DIS_TWAI_W {
        DIS_TWAI_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dis_force_download(&mut self) -> DIS_FORCE_DOWNLOAD_W {
        DIS_FORCE_DOWNLOAD_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dis_usb_device(&mut self) -> DIS_USB_DEVICE_W {
        DIS_USB_DEVICE_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dis_download_icache(&mut self) -> DIS_DOWNLOAD_ICACHE_W {
        DIS_DOWNLOAD_ICACHE_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dis_usb_jtag(&mut self) -> DIS_USB_JTAG_W {
        DIS_USB_JTAG_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dis_icache(&mut self) -> DIS_ICACHE_W {
        DIS_ICACHE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dis_rtc_ram_boot(&mut self) -> DIS_RTC_RAM_BOOT_W {
        DIS_RTC_RAM_BOOT_W { w: self }
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn rd_dis(&mut self) -> RD_DIS_W {
        RD_DIS_W { w: self }
    }
}
