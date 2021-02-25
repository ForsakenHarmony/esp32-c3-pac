#[doc = "Reader of register RTC_CNTL"]
pub type R = crate::R<u32, super::RTC_CNTL>;
#[doc = "Writer for register RTC_CNTL"]
pub type W = crate::W<u32, super::RTC_CNTL>;
#[doc = "Register RTC_CNTL `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REGULATOR_FORCE_PU`"]
pub type REGULATOR_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REGULATOR_FORCE_PU`"]
pub struct REGULATOR_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> REGULATOR_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `REGULATOR_FORCE_PD`"]
pub type REGULATOR_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REGULATOR_FORCE_PD`"]
pub struct REGULATOR_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> REGULATOR_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `DBOOST_FORCE_PU`"]
pub type DBOOST_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBOOST_FORCE_PU`"]
pub struct DBOOST_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> DBOOST_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `DBOOST_FORCE_PD`"]
pub type DBOOST_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBOOST_FORCE_PD`"]
pub struct DBOOST_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> DBOOST_FORCE_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn regulator_force_pu(&self) -> REGULATOR_FORCE_PU_R {
        REGULATOR_FORCE_PU_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn regulator_force_pd(&self) -> REGULATOR_FORCE_PD_R {
        REGULATOR_FORCE_PD_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dboost_force_pu(&self) -> DBOOST_FORCE_PU_R {
        DBOOST_FORCE_PU_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dboost_force_pd(&self) -> DBOOST_FORCE_PD_R {
        DBOOST_FORCE_PD_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn regulator_force_pu(&mut self) -> REGULATOR_FORCE_PU_W {
        REGULATOR_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn regulator_force_pd(&mut self) -> REGULATOR_FORCE_PD_W {
        REGULATOR_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dboost_force_pu(&mut self) -> DBOOST_FORCE_PU_W {
        DBOOST_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dboost_force_pd(&mut self) -> DBOOST_FORCE_PD_W {
        DBOOST_FORCE_PD_W { w: self }
    }
}
