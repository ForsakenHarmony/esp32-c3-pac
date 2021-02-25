#[doc = "Reader of register DBG_SEL"]
pub type R = crate::R<u32, super::DBG_SEL>;
#[doc = "Writer for register DBG_SEL"]
pub type W = crate::W<u32, super::DBG_SEL>;
#[doc = "Register DBG_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::DBG_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DEBUG_SEL4`"]
pub type DEBUG_SEL4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DEBUG_SEL4`"]
pub struct DEBUG_SEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUG_SEL4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | (((value as u32) & 0x1f) << 27);
        self.w
    }
}
#[doc = "Reader of field `DEBUG_SEL3`"]
pub type DEBUG_SEL3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DEBUG_SEL3`"]
pub struct DEBUG_SEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUG_SEL3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 22)) | (((value as u32) & 0x1f) << 22);
        self.w
    }
}
#[doc = "Reader of field `DEBUG_SEL2`"]
pub type DEBUG_SEL2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DEBUG_SEL2`"]
pub struct DEBUG_SEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUG_SEL2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 17)) | (((value as u32) & 0x1f) << 17);
        self.w
    }
}
#[doc = "Reader of field `DEBUG_SEL1`"]
pub type DEBUG_SEL1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DEBUG_SEL1`"]
pub struct DEBUG_SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUG_SEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 12)) | (((value as u32) & 0x1f) << 12);
        self.w
    }
}
#[doc = "Reader of field `DEBUG_SEL0`"]
pub type DEBUG_SEL0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DEBUG_SEL0`"]
pub struct DEBUG_SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUG_SEL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 7)) | (((value as u32) & 0x1f) << 7);
        self.w
    }
}
#[doc = "Reader of field `DEBUG_BIT_SEL`"]
pub type DEBUG_BIT_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DEBUG_BIT_SEL`"]
pub struct DEBUG_BIT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUG_BIT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 2)) | (((value as u32) & 0x1f) << 2);
        self.w
    }
}
#[doc = "Reader of field `DEBUG_12M_NO_GATING`"]
pub type DEBUG_12M_NO_GATING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEBUG_12M_NO_GATING`"]
pub struct DEBUG_12M_NO_GATING_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUG_12M_NO_GATING_W<'a> {
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
impl R {
    #[doc = "Bits 27:31"]
    #[inline(always)]
    pub fn debug_sel4(&self) -> DEBUG_SEL4_R {
        DEBUG_SEL4_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
    #[doc = "Bits 22:26"]
    #[inline(always)]
    pub fn debug_sel3(&self) -> DEBUG_SEL3_R {
        DEBUG_SEL3_R::new(((self.bits >> 22) & 0x1f) as u8)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn debug_sel2(&self) -> DEBUG_SEL2_R {
        DEBUG_SEL2_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16"]
    #[inline(always)]
    pub fn debug_sel1(&self) -> DEBUG_SEL1_R {
        DEBUG_SEL1_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 7:11"]
    #[inline(always)]
    pub fn debug_sel0(&self) -> DEBUG_SEL0_R {
        DEBUG_SEL0_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 2:6"]
    #[inline(always)]
    pub fn debug_bit_sel(&self) -> DEBUG_BIT_SEL_R {
        DEBUG_BIT_SEL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn debug_12m_no_gating(&self) -> DEBUG_12M_NO_GATING_R {
        DEBUG_12M_NO_GATING_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 27:31"]
    #[inline(always)]
    pub fn debug_sel4(&mut self) -> DEBUG_SEL4_W {
        DEBUG_SEL4_W { w: self }
    }
    #[doc = "Bits 22:26"]
    #[inline(always)]
    pub fn debug_sel3(&mut self) -> DEBUG_SEL3_W {
        DEBUG_SEL3_W { w: self }
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn debug_sel2(&mut self) -> DEBUG_SEL2_W {
        DEBUG_SEL2_W { w: self }
    }
    #[doc = "Bits 12:16"]
    #[inline(always)]
    pub fn debug_sel1(&mut self) -> DEBUG_SEL1_W {
        DEBUG_SEL1_W { w: self }
    }
    #[doc = "Bits 7:11"]
    #[inline(always)]
    pub fn debug_sel0(&mut self) -> DEBUG_SEL0_W {
        DEBUG_SEL0_W { w: self }
    }
    #[doc = "Bits 2:6"]
    #[inline(always)]
    pub fn debug_bit_sel(&mut self) -> DEBUG_BIT_SEL_W {
        DEBUG_BIT_SEL_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn debug_12m_no_gating(&mut self) -> DEBUG_12M_NO_GATING_W {
        DEBUG_12M_NO_GATING_W { w: self }
    }
}
