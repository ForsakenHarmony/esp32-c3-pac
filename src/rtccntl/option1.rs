#[doc = "Reader of register OPTION1"]
pub type R = crate::R<u32, super::OPTION1>;
#[doc = "Writer for register OPTION1"]
pub type W = crate::W<u32, super::OPTION1>;
#[doc = "Register OPTION1 `reset()`'s with value 0"]
impl crate::ResetValue for super::OPTION1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FORCE_DOWNLOAD_BOOT`"]
pub type FORCE_DOWNLOAD_BOOT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE_DOWNLOAD_BOOT`"]
pub struct FORCE_DOWNLOAD_BOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_DOWNLOAD_BOOT_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn force_download_boot(&self) -> FORCE_DOWNLOAD_BOOT_R {
        FORCE_DOWNLOAD_BOOT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn force_download_boot(&mut self) -> FORCE_DOWNLOAD_BOOT_W {
        FORCE_DOWNLOAD_BOOT_W { w: self }
    }
}
