#[doc = "Reader of register PGM_DATA2"]
pub type R = crate::R<u32, super::PGM_DATA2>;
#[doc = "Writer for register PGM_DATA2"]
pub type W = crate::W<u32, super::PGM_DATA2>;
#[doc = "Register PGM_DATA2 `reset()`'s with value 0"]
impl crate::ResetValue for super::PGM_DATA2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `KEY_PURPOSE_1`"]
pub type KEY_PURPOSE_1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `KEY_PURPOSE_1`"]
pub struct KEY_PURPOSE_1_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_PURPOSE_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `KEY_PURPOSE_0`"]
pub type KEY_PURPOSE_0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `KEY_PURPOSE_0`"]
pub struct KEY_PURPOSE_0_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_PURPOSE_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `SECURE_BOOT_KEY_REVOKE2`"]
pub type SECURE_BOOT_KEY_REVOKE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SECURE_BOOT_KEY_REVOKE2`"]
pub struct SECURE_BOOT_KEY_REVOKE2_W<'a> {
    w: &'a mut W,
}
impl<'a> SECURE_BOOT_KEY_REVOKE2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `SECURE_BOOT_KEY_REVOKE1`"]
pub type SECURE_BOOT_KEY_REVOKE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SECURE_BOOT_KEY_REVOKE1`"]
pub struct SECURE_BOOT_KEY_REVOKE1_W<'a> {
    w: &'a mut W,
}
impl<'a> SECURE_BOOT_KEY_REVOKE1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `SECURE_BOOT_KEY_REVOKE0`"]
pub type SECURE_BOOT_KEY_REVOKE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SECURE_BOOT_KEY_REVOKE0`"]
pub struct SECURE_BOOT_KEY_REVOKE0_W<'a> {
    w: &'a mut W,
}
impl<'a> SECURE_BOOT_KEY_REVOKE0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `SPI_BOOT_CRYPT_CNT`"]
pub type SPI_BOOT_CRYPT_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_BOOT_CRYPT_CNT`"]
pub struct SPI_BOOT_CRYPT_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_BOOT_CRYPT_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Reader of field `WAT_DELAY_SEL`"]
pub type WAT_DELAY_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WAT_DELAY_SEL`"]
pub struct WAT_DELAY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WAT_DELAY_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `RPT4_RESERVED2`"]
pub type RPT4_RESERVED2_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn key_purpose_1(&self) -> KEY_PURPOSE_1_R {
        KEY_PURPOSE_1_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn key_purpose_0(&self) -> KEY_PURPOSE_0_R {
        KEY_PURPOSE_0_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn secure_boot_key_revoke2(&self) -> SECURE_BOOT_KEY_REVOKE2_R {
        SECURE_BOOT_KEY_REVOKE2_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn secure_boot_key_revoke1(&self) -> SECURE_BOOT_KEY_REVOKE1_R {
        SECURE_BOOT_KEY_REVOKE1_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn secure_boot_key_revoke0(&self) -> SECURE_BOOT_KEY_REVOKE0_R {
        SECURE_BOOT_KEY_REVOKE0_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn spi_boot_crypt_cnt(&self) -> SPI_BOOT_CRYPT_CNT_R {
        SPI_BOOT_CRYPT_CNT_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn wat_delay_sel(&self) -> WAT_DELAY_SEL_R {
        WAT_DELAY_SEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rpt4_reserved2(&self) -> RPT4_RESERVED2_R {
        RPT4_RESERVED2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn key_purpose_1(&mut self) -> KEY_PURPOSE_1_W {
        KEY_PURPOSE_1_W { w: self }
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn key_purpose_0(&mut self) -> KEY_PURPOSE_0_W {
        KEY_PURPOSE_0_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn secure_boot_key_revoke2(&mut self) -> SECURE_BOOT_KEY_REVOKE2_W {
        SECURE_BOOT_KEY_REVOKE2_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn secure_boot_key_revoke1(&mut self) -> SECURE_BOOT_KEY_REVOKE1_W {
        SECURE_BOOT_KEY_REVOKE1_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn secure_boot_key_revoke0(&mut self) -> SECURE_BOOT_KEY_REVOKE0_W {
        SECURE_BOOT_KEY_REVOKE0_W { w: self }
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn spi_boot_crypt_cnt(&mut self) -> SPI_BOOT_CRYPT_CNT_W {
        SPI_BOOT_CRYPT_CNT_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn wat_delay_sel(&mut self) -> WAT_DELAY_SEL_W {
        WAT_DELAY_SEL_W { w: self }
    }
}
