#[doc = "Reader of register CLK"]
pub type R = crate::R<u32, super::CLK>;
#[doc = "Writer for register CLK"]
pub type W = crate::W<u32, super::CLK>;
#[doc = "Register CLK `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLK_EN`"]
pub type CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK_EN`"]
pub struct CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `MEM_FORCE_PU`"]
pub type MEM_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MEM_FORCE_PU`"]
pub struct MEM_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `MEM_CLK_FORCE_ON`"]
pub type MEM_CLK_FORCE_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MEM_CLK_FORCE_ON`"]
pub struct MEM_CLK_FORCE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_CLK_FORCE_ON_W<'a> {
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
#[doc = "Reader of field `MEM_FORCE_PD`"]
pub type MEM_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MEM_FORCE_PD`"]
pub struct MEM_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_FORCE_PD_W<'a> {
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
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn mem_force_pu(&self) -> MEM_FORCE_PU_R {
        MEM_FORCE_PU_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn mem_clk_force_on(&self) -> MEM_CLK_FORCE_ON_R {
        MEM_CLK_FORCE_ON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mem_force_pd(&self) -> MEM_FORCE_PD_R {
        MEM_FORCE_PD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W {
        CLK_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn mem_force_pu(&mut self) -> MEM_FORCE_PU_W {
        MEM_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn mem_clk_force_on(&mut self) -> MEM_CLK_FORCE_ON_W {
        MEM_CLK_FORCE_ON_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mem_force_pd(&mut self) -> MEM_FORCE_PD_W {
        MEM_FORCE_PD_W { w: self }
    }
}
