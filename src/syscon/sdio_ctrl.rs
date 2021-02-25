#[doc = "Reader of register SDIO_CTRL"]
pub type R = crate::R<u32, super::SDIO_CTRL>;
#[doc = "Writer for register SDIO_CTRL"]
pub type W = crate::W<u32, super::SDIO_CTRL>;
#[doc = "Register SDIO_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SDIO_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SDIO_WIN_ACCESS_EN`"]
pub type SDIO_WIN_ACCESS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDIO_WIN_ACCESS_EN`"]
pub struct SDIO_WIN_ACCESS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_WIN_ACCESS_EN_W<'a> {
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
    pub fn sdio_win_access_en(&self) -> SDIO_WIN_ACCESS_EN_R {
        SDIO_WIN_ACCESS_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sdio_win_access_en(&mut self) -> SDIO_WIN_ACCESS_EN_W {
        SDIO_WIN_ACCESS_EN_W { w: self }
    }
}
