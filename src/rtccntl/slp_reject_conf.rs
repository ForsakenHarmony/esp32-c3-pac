#[doc = "Reader of register SLP_REJECT_CONF"]
pub type R = crate::R<u32, super::SLP_REJECT_CONF>;
#[doc = "Writer for register SLP_REJECT_CONF"]
pub type W = crate::W<u32, super::SLP_REJECT_CONF>;
#[doc = "Register SLP_REJECT_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::SLP_REJECT_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DEEP_SLP_REJECT_EN`"]
pub type DEEP_SLP_REJECT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEEP_SLP_REJECT_EN`"]
pub struct DEEP_SLP_REJECT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEEP_SLP_REJECT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `LIGHT_SLP_REJECT_EN`"]
pub type LIGHT_SLP_REJECT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LIGHT_SLP_REJECT_EN`"]
pub struct LIGHT_SLP_REJECT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LIGHT_SLP_REJECT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `SLEEP_REJECT_ENA`"]
pub type SLEEP_REJECT_ENA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SLEEP_REJECT_ENA`"]
pub struct SLEEP_REJECT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEP_REJECT_ENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 12)) | (((value as u32) & 0x0003_ffff) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn deep_slp_reject_en(&self) -> DEEP_SLP_REJECT_EN_R {
        DEEP_SLP_REJECT_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn light_slp_reject_en(&self) -> LIGHT_SLP_REJECT_EN_R {
        LIGHT_SLP_REJECT_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 12:29"]
    #[inline(always)]
    pub fn sleep_reject_ena(&self) -> SLEEP_REJECT_ENA_R {
        SLEEP_REJECT_ENA_R::new(((self.bits >> 12) & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn deep_slp_reject_en(&mut self) -> DEEP_SLP_REJECT_EN_W {
        DEEP_SLP_REJECT_EN_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn light_slp_reject_en(&mut self) -> LIGHT_SLP_REJECT_EN_W {
        LIGHT_SLP_REJECT_EN_W { w: self }
    }
    #[doc = "Bits 12:29"]
    #[inline(always)]
    pub fn sleep_reject_ena(&mut self) -> SLEEP_REJECT_ENA_W {
        SLEEP_REJECT_ENA_W { w: self }
    }
}
