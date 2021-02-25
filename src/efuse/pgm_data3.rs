#[doc = "Reader of register PGM_DATA3"]
pub type R = crate::R<u32, super::PGM_DATA3>;
#[doc = "Writer for register PGM_DATA3"]
pub type W = crate::W<u32, super::PGM_DATA3>;
#[doc = "Register PGM_DATA3 `reset()`'s with value 0"]
impl crate::ResetValue for super::PGM_DATA3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FLASH_TPUW`"]
pub type FLASH_TPUW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FLASH_TPUW`"]
pub struct FLASH_TPUW_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_TPUW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `RPT4_RESERVED0`"]
pub type RPT4_RESERVED0_R = crate::R<u8, u8>;
#[doc = "Reader of field `SECURE_BOOT_AGGRESSIVE_REVOKE`"]
pub type SECURE_BOOT_AGGRESSIVE_REVOKE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SECURE_BOOT_AGGRESSIVE_REVOKE`"]
pub struct SECURE_BOOT_AGGRESSIVE_REVOKE_W<'a> {
    w: &'a mut W,
}
impl<'a> SECURE_BOOT_AGGRESSIVE_REVOKE_W<'a> {
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
#[doc = "Reader of field `SECURE_BOOT_EN`"]
pub type SECURE_BOOT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SECURE_BOOT_EN`"]
pub struct SECURE_BOOT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SECURE_BOOT_EN_W<'a> {
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
#[doc = "Reader of field `RPT4_RESERVED3`"]
pub type RPT4_RESERVED3_R = crate::R<u8, u8>;
#[doc = "Reader of field `KEY_PURPOSE_5`"]
pub type KEY_PURPOSE_5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `KEY_PURPOSE_5`"]
pub struct KEY_PURPOSE_5_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_PURPOSE_5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `KEY_PURPOSE_4`"]
pub type KEY_PURPOSE_4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `KEY_PURPOSE_4`"]
pub struct KEY_PURPOSE_4_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_PURPOSE_4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `KEY_PURPOSE_3`"]
pub type KEY_PURPOSE_3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `KEY_PURPOSE_3`"]
pub struct KEY_PURPOSE_3_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_PURPOSE_3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `KEY_PURPOSE_2`"]
pub type KEY_PURPOSE_2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `KEY_PURPOSE_2`"]
pub struct KEY_PURPOSE_2_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_PURPOSE_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn flash_tpuw(&self) -> FLASH_TPUW_R {
        FLASH_TPUW_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 22:27"]
    #[inline(always)]
    pub fn rpt4_reserved0(&self) -> RPT4_RESERVED0_R {
        RPT4_RESERVED0_R::new(((self.bits >> 22) & 0x3f) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn secure_boot_aggressive_revoke(&self) -> SECURE_BOOT_AGGRESSIVE_REVOKE_R {
        SECURE_BOOT_AGGRESSIVE_REVOKE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn secure_boot_en(&self) -> SECURE_BOOT_EN_R {
        SECURE_BOOT_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn rpt4_reserved3(&self) -> RPT4_RESERVED3_R {
        RPT4_RESERVED3_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn key_purpose_5(&self) -> KEY_PURPOSE_5_R {
        KEY_PURPOSE_5_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn key_purpose_4(&self) -> KEY_PURPOSE_4_R {
        KEY_PURPOSE_4_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn key_purpose_3(&self) -> KEY_PURPOSE_3_R {
        KEY_PURPOSE_3_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn key_purpose_2(&self) -> KEY_PURPOSE_2_R {
        KEY_PURPOSE_2_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn flash_tpuw(&mut self) -> FLASH_TPUW_W {
        FLASH_TPUW_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn secure_boot_aggressive_revoke(&mut self) -> SECURE_BOOT_AGGRESSIVE_REVOKE_W {
        SECURE_BOOT_AGGRESSIVE_REVOKE_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn secure_boot_en(&mut self) -> SECURE_BOOT_EN_W {
        SECURE_BOOT_EN_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn key_purpose_5(&mut self) -> KEY_PURPOSE_5_W {
        KEY_PURPOSE_5_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn key_purpose_4(&mut self) -> KEY_PURPOSE_4_W {
        KEY_PURPOSE_4_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn key_purpose_3(&mut self) -> KEY_PURPOSE_3_W {
        KEY_PURPOSE_3_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn key_purpose_2(&mut self) -> KEY_PURPOSE_2_W {
        KEY_PURPOSE_2_W { w: self }
    }
}
