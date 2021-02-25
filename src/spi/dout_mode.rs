#[doc = "Reader of register DOUT_MODE"]
pub type R = crate::R<u32, super::DOUT_MODE>;
#[doc = "Writer for register DOUT_MODE"]
pub type W = crate::W<u32, super::DOUT_MODE>;
#[doc = "Register DOUT_MODE `reset()`'s with value 0"]
impl crate::ResetValue for super::DOUT_MODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DOUT3_MODE`"]
pub type DOUT3_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOUT3_MODE`"]
pub struct DOUT3_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT3_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `DOUT2_MODE`"]
pub type DOUT2_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOUT2_MODE`"]
pub struct DOUT2_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT2_MODE_W<'a> {
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
#[doc = "Reader of field `DOUT1_MODE`"]
pub type DOUT1_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOUT1_MODE`"]
pub struct DOUT1_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT1_MODE_W<'a> {
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
#[doc = "Reader of field `DOUT0_MODE`"]
pub type DOUT0_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOUT0_MODE`"]
pub struct DOUT0_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT0_MODE_W<'a> {
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
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dout3_mode(&self) -> DOUT3_MODE_R {
        DOUT3_MODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dout2_mode(&self) -> DOUT2_MODE_R {
        DOUT2_MODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dout1_mode(&self) -> DOUT1_MODE_R {
        DOUT1_MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dout0_mode(&self) -> DOUT0_MODE_R {
        DOUT0_MODE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dout3_mode(&mut self) -> DOUT3_MODE_W {
        DOUT3_MODE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dout2_mode(&mut self) -> DOUT2_MODE_W {
        DOUT2_MODE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dout1_mode(&mut self) -> DOUT1_MODE_W {
        DOUT1_MODE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dout0_mode(&mut self) -> DOUT0_MODE_W {
        DOUT0_MODE_W { w: self }
    }
}
