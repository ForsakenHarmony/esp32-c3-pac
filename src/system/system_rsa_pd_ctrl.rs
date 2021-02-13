#[doc = "Reader of register SYSTEM_RSA_PD_CTRL"]
pub type R = crate::R<u32, super::SYSTEM_RSA_PD_CTRL>;
#[doc = "Writer for register SYSTEM_RSA_PD_CTRL"]
pub type W = crate::W<u32, super::SYSTEM_RSA_PD_CTRL>;
#[doc = "Register SYSTEM_RSA_PD_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSTEM_RSA_PD_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSTEM_RSA_MEM_FORCE_PD`"]
pub type SYSTEM_RSA_MEM_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_RSA_MEM_FORCE_PD`"]
pub struct SYSTEM_RSA_MEM_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_RSA_MEM_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `SYSTEM_RSA_MEM_FORCE_PU`"]
pub type SYSTEM_RSA_MEM_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_RSA_MEM_FORCE_PU`"]
pub struct SYSTEM_RSA_MEM_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_RSA_MEM_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `SYSTEM_RSA_MEM_PD`"]
pub type SYSTEM_RSA_MEM_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_RSA_MEM_PD`"]
pub struct SYSTEM_RSA_MEM_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_RSA_MEM_PD_W<'a> {
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
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn system_rsa_mem_force_pd(&self) -> SYSTEM_RSA_MEM_FORCE_PD_R {
        SYSTEM_RSA_MEM_FORCE_PD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn system_rsa_mem_force_pu(&self) -> SYSTEM_RSA_MEM_FORCE_PU_R {
        SYSTEM_RSA_MEM_FORCE_PU_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn system_rsa_mem_pd(&self) -> SYSTEM_RSA_MEM_PD_R {
        SYSTEM_RSA_MEM_PD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn system_rsa_mem_force_pd(&mut self) -> SYSTEM_RSA_MEM_FORCE_PD_W {
        SYSTEM_RSA_MEM_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn system_rsa_mem_force_pu(&mut self) -> SYSTEM_RSA_MEM_FORCE_PU_W {
        SYSTEM_RSA_MEM_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn system_rsa_mem_pd(&mut self) -> SYSTEM_RSA_MEM_PD_W {
        SYSTEM_RSA_MEM_PD_W { w: self }
    }
}
