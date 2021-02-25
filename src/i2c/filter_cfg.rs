#[doc = "Reader of register FILTER_CFG"]
pub type R = crate::R<u32, super::FILTER_CFG>;
#[doc = "Writer for register FILTER_CFG"]
pub type W = crate::W<u32, super::FILTER_CFG>;
#[doc = "Register FILTER_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::FILTER_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SDA_FILTER_EN`"]
pub type SDA_FILTER_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDA_FILTER_EN`"]
pub struct SDA_FILTER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDA_FILTER_EN_W<'a> {
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
#[doc = "Reader of field `SCL_FILTER_EN`"]
pub type SCL_FILTER_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCL_FILTER_EN`"]
pub struct SCL_FILTER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_FILTER_EN_W<'a> {
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
#[doc = "Reader of field `SDA_FILTER_THRES`"]
pub type SDA_FILTER_THRES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDA_FILTER_THRES`"]
pub struct SDA_FILTER_THRES_W<'a> {
    w: &'a mut W,
}
impl<'a> SDA_FILTER_THRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `SCL_FILTER_THRES`"]
pub type SCL_FILTER_THRES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCL_FILTER_THRES`"]
pub struct SCL_FILTER_THRES_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_FILTER_THRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sda_filter_en(&self) -> SDA_FILTER_EN_R {
        SDA_FILTER_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn scl_filter_en(&self) -> SCL_FILTER_EN_R {
        SCL_FILTER_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn sda_filter_thres(&self) -> SDA_FILTER_THRES_R {
        SDA_FILTER_THRES_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn scl_filter_thres(&self) -> SCL_FILTER_THRES_R {
        SCL_FILTER_THRES_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sda_filter_en(&mut self) -> SDA_FILTER_EN_W {
        SDA_FILTER_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn scl_filter_en(&mut self) -> SCL_FILTER_EN_W {
        SCL_FILTER_EN_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn sda_filter_thres(&mut self) -> SDA_FILTER_THRES_W {
        SDA_FILTER_THRES_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn scl_filter_thres(&mut self) -> SCL_FILTER_THRES_W {
        SCL_FILTER_THRES_W { w: self }
    }
}
