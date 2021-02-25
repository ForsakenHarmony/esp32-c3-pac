#[doc = "Reader of register PGM_DATA4"]
pub type R = crate::R<u32, super::PGM_DATA4>;
#[doc = "Writer for register PGM_DATA4"]
pub type W = crate::W<u32, super::PGM_DATA4>;
#[doc = "Register PGM_DATA4 `reset()`'s with value 0"]
impl crate::ResetValue for super::PGM_DATA4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RPT4_RESERVED1`"]
pub type RPT4_RESERVED1_R = crate::R<u8, u8>;
#[doc = "Reader of field `SECURE_VERSION`"]
pub type SECURE_VERSION_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SECURE_VERSION`"]
pub struct SECURE_VERSION_W<'a> {
    w: &'a mut W,
}
impl<'a> SECURE_VERSION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 14)) | (((value as u32) & 0xffff) << 14);
        self.w
    }
}
#[doc = "Reader of field `FORCE_SEND_RESUME`"]
pub type FORCE_SEND_RESUME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE_SEND_RESUME`"]
pub struct FORCE_SEND_RESUME_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_SEND_RESUME_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `FLASH_ECC_EN`"]
pub type FLASH_ECC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH_ECC_EN`"]
pub struct FLASH_ECC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_ECC_EN_W<'a> {
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
#[doc = "Reader of field `FLASH_PAGE_SIZE`"]
pub type FLASH_PAGE_SIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FLASH_PAGE_SIZE`"]
pub struct FLASH_PAGE_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_PAGE_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `FLASH_TYPE`"]
pub type FLASH_TYPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH_TYPE`"]
pub struct FLASH_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_TYPE_W<'a> {
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
#[doc = "Reader of field `PIN_POWER_SELECTION`"]
pub type PIN_POWER_SELECTION_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PIN_POWER_SELECTION`"]
pub struct PIN_POWER_SELECTION_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_POWER_SELECTION_W<'a> {
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
#[doc = "Reader of field `UART_PRINT_CONTROL`"]
pub type UART_PRINT_CONTROL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART_PRINT_CONTROL`"]
pub struct UART_PRINT_CONTROL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_PRINT_CONTROL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `ENABLE_SECURITY_DOWNLOAD`"]
pub type ENABLE_SECURITY_DOWNLOAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE_SECURITY_DOWNLOAD`"]
pub struct ENABLE_SECURITY_DOWNLOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_SECURITY_DOWNLOAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `DIS_USB_DOWNLOAD_MODE`"]
pub type DIS_USB_DOWNLOAD_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_USB_DOWNLOAD_MODE`"]
pub struct DIS_USB_DOWNLOAD_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_USB_DOWNLOAD_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `FLASH_ECC_MODE`"]
pub type FLASH_ECC_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH_ECC_MODE`"]
pub struct FLASH_ECC_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_ECC_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `UART_PRINT_CHANNEL`"]
pub type UART_PRINT_CHANNEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_PRINT_CHANNEL`"]
pub struct UART_PRINT_CHANNEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_PRINT_CHANNEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `DIS_LEGACY_SPI_BOOT`"]
pub type DIS_LEGACY_SPI_BOOT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_LEGACY_SPI_BOOT`"]
pub struct DIS_LEGACY_SPI_BOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_LEGACY_SPI_BOOT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `DIS_DOWNLOAD_MODE`"]
pub type DIS_DOWNLOAD_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_DOWNLOAD_MODE`"]
pub struct DIS_DOWNLOAD_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_DOWNLOAD_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
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
impl W {
    #[doc = "Bits 14:29"]
    #[inline(always)]
    pub fn secure_version(&mut self) -> SECURE_VERSION_W {
        SECURE_VERSION_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn force_send_resume(&mut self) -> FORCE_SEND_RESUME_W {
        FORCE_SEND_RESUME_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn flash_ecc_en(&mut self) -> FLASH_ECC_EN_W {
        FLASH_ECC_EN_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn flash_page_size(&mut self) -> FLASH_PAGE_SIZE_W {
        FLASH_PAGE_SIZE_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn flash_type(&mut self) -> FLASH_TYPE_W {
        FLASH_TYPE_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pin_power_selection(&mut self) -> PIN_POWER_SELECTION_W {
        PIN_POWER_SELECTION_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn uart_print_control(&mut self) -> UART_PRINT_CONTROL_W {
        UART_PRINT_CONTROL_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn enable_security_download(&mut self) -> ENABLE_SECURITY_DOWNLOAD_W {
        ENABLE_SECURITY_DOWNLOAD_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dis_usb_download_mode(&mut self) -> DIS_USB_DOWNLOAD_MODE_W {
        DIS_USB_DOWNLOAD_MODE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn flash_ecc_mode(&mut self) -> FLASH_ECC_MODE_W {
        FLASH_ECC_MODE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn uart_print_channel(&mut self) -> UART_PRINT_CHANNEL_W {
        UART_PRINT_CHANNEL_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dis_legacy_spi_boot(&mut self) -> DIS_LEGACY_SPI_BOOT_W {
        DIS_LEGACY_SPI_BOOT_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dis_download_mode(&mut self) -> DIS_DOWNLOAD_MODE_W {
        DIS_DOWNLOAD_MODE_W { w: self }
    }
}
