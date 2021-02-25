#[doc = "Reader of register SCL_SP_CONF"]
pub type R = crate::R<u32, super::SCL_SP_CONF>;
#[doc = "Writer for register SCL_SP_CONF"]
pub type W = crate::W<u32, super::SCL_SP_CONF>;
#[doc = "Register SCL_SP_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::SCL_SP_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SDA_PD_EN`"]
pub type SDA_PD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDA_PD_EN`"]
pub struct SDA_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDA_PD_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `SCL_PD_EN`"]
pub type SCL_PD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCL_PD_EN`"]
pub struct SCL_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_PD_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `SCL_RST_SLV_NUM`"]
pub type SCL_RST_SLV_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCL_RST_SLV_NUM`"]
pub struct SCL_RST_SLV_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_RST_SLV_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 1)) | (((value as u32) & 0x1f) << 1);
        self.w
    }
}
#[doc = "Reader of field `SCL_RST_SLV_EN`"]
pub type SCL_RST_SLV_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCL_RST_SLV_EN`"]
pub struct SCL_RST_SLV_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_RST_SLV_EN_W<'a> {
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
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sda_pd_en(&self) -> SDA_PD_EN_R {
        SDA_PD_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn scl_pd_en(&self) -> SCL_PD_EN_R {
        SCL_PD_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 1:5"]
    #[inline(always)]
    pub fn scl_rst_slv_num(&self) -> SCL_RST_SLV_NUM_R {
        SCL_RST_SLV_NUM_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn scl_rst_slv_en(&self) -> SCL_RST_SLV_EN_R {
        SCL_RST_SLV_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sda_pd_en(&mut self) -> SDA_PD_EN_W {
        SDA_PD_EN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn scl_pd_en(&mut self) -> SCL_PD_EN_W {
        SCL_PD_EN_W { w: self }
    }
    #[doc = "Bits 1:5"]
    #[inline(always)]
    pub fn scl_rst_slv_num(&mut self) -> SCL_RST_SLV_NUM_W {
        SCL_RST_SLV_NUM_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn scl_rst_slv_en(&mut self) -> SCL_RST_SLV_EN_W {
        SCL_RST_SLV_EN_W { w: self }
    }
}
