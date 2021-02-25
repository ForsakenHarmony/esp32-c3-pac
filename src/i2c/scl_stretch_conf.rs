#[doc = "Reader of register SCL_STRETCH_CONF"]
pub type R = crate::R<u32, super::SCL_STRETCH_CONF>;
#[doc = "Writer for register SCL_STRETCH_CONF"]
pub type W = crate::W<u32, super::SCL_STRETCH_CONF>;
#[doc = "Register SCL_STRETCH_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::SCL_STRETCH_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLAVE_BYTE_ACK_LVL`"]
pub type SLAVE_BYTE_ACK_LVL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLAVE_BYTE_ACK_LVL`"]
pub struct SLAVE_BYTE_ACK_LVL_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_BYTE_ACK_LVL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `SLAVE_BYTE_ACK_CTL_EN`"]
pub type SLAVE_BYTE_ACK_CTL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLAVE_BYTE_ACK_CTL_EN`"]
pub struct SLAVE_BYTE_ACK_CTL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_BYTE_ACK_CTL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Write proxy for field `SLAVE_SCL_STRETCH_CLR`"]
pub struct SLAVE_SCL_STRETCH_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_SCL_STRETCH_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `SLAVE_SCL_STRETCH_EN`"]
pub type SLAVE_SCL_STRETCH_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLAVE_SCL_STRETCH_EN`"]
pub struct SLAVE_SCL_STRETCH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_SCL_STRETCH_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `STRETCH_PROTECT_NUM`"]
pub type STRETCH_PROTECT_NUM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `STRETCH_PROTECT_NUM`"]
pub struct STRETCH_PROTECT_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> STRETCH_PROTECT_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn slave_byte_ack_lvl(&self) -> SLAVE_BYTE_ACK_LVL_R {
        SLAVE_BYTE_ACK_LVL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slave_byte_ack_ctl_en(&self) -> SLAVE_BYTE_ACK_CTL_EN_R {
        SLAVE_BYTE_ACK_CTL_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slave_scl_stretch_en(&self) -> SLAVE_SCL_STRETCH_EN_R {
        SLAVE_SCL_STRETCH_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn stretch_protect_num(&self) -> STRETCH_PROTECT_NUM_R {
        STRETCH_PROTECT_NUM_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn slave_byte_ack_lvl(&mut self) -> SLAVE_BYTE_ACK_LVL_W {
        SLAVE_BYTE_ACK_LVL_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slave_byte_ack_ctl_en(&mut self) -> SLAVE_BYTE_ACK_CTL_EN_W {
        SLAVE_BYTE_ACK_CTL_EN_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slave_scl_stretch_clr(&mut self) -> SLAVE_SCL_STRETCH_CLR_W {
        SLAVE_SCL_STRETCH_CLR_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slave_scl_stretch_en(&mut self) -> SLAVE_SCL_STRETCH_EN_W {
        SLAVE_SCL_STRETCH_EN_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn stretch_protect_num(&mut self) -> STRETCH_PROTECT_NUM_W {
        STRETCH_PROTECT_NUM_W { w: self }
    }
}
