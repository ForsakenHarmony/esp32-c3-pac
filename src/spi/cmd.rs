#[doc = "Reader of register CMD"]
pub type R = crate::R<u32, super::CMD>;
#[doc = "Writer for register CMD"]
pub type W = crate::W<u32, super::CMD>;
#[doc = "Register CMD `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USR`"]
pub type USR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USR`"]
pub struct USR_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Write proxy for field `UPDATE`"]
pub struct UPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDATE_W<'a> {
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
#[doc = "Reader of field `CONF_BITLEN`"]
pub type CONF_BITLEN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CONF_BITLEN`"]
pub struct CONF_BITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CONF_BITLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | ((value as u32) & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn usr(&self) -> USR_R {
        USR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 0:17"]
    #[inline(always)]
    pub fn conf_bitlen(&self) -> CONF_BITLEN_R {
        CONF_BITLEN_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn usr(&mut self) -> USR_W {
        USR_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn update(&mut self) -> UPDATE_W {
        UPDATE_W { w: self }
    }
    #[doc = "Bits 0:17"]
    #[inline(always)]
    pub fn conf_bitlen(&mut self) -> CONF_BITLEN_W {
        CONF_BITLEN_W { w: self }
    }
}
