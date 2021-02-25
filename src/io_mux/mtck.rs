#[doc = "Reader of register MTCK"]
pub type R = crate::R<u32, super::MTCK>;
#[doc = "Writer for register MTCK"]
pub type W = crate::W<u32, super::MTCK>;
#[doc = "Register MTCK `reset()`'s with value 0x0a00"]
impl crate::ResetValue for super::MTCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0a00
    }
}
#[doc = "Reader of field `MCU_SEL`"]
pub type MCU_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCU_SEL`"]
pub struct MCU_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MCU_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `FUN_DRV`"]
pub type FUN_DRV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FUN_DRV`"]
pub struct FUN_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> FUN_DRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `FUN_IE`"]
pub type FUN_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FUN_IE`"]
pub struct FUN_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> FUN_IE_W<'a> {
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
#[doc = "Reader of field `FUN_PU`"]
pub type FUN_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FUN_PU`"]
pub struct FUN_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> FUN_PU_W<'a> {
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
#[doc = "Reader of field `FUN_PD`"]
pub type FUN_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FUN_PD`"]
pub struct FUN_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> FUN_PD_W<'a> {
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
#[doc = "Reader of field `SLP_DRV`"]
pub type SLP_DRV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLP_DRV`"]
pub struct SLP_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_DRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `SLP_IE`"]
pub type SLP_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLP_IE`"]
pub struct SLP_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `SLP_PU`"]
pub type SLP_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLP_PU`"]
pub struct SLP_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_PU_W<'a> {
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
#[doc = "Reader of field `SLP_PD`"]
pub type SLP_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLP_PD`"]
pub struct SLP_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_PD_W<'a> {
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
#[doc = "Reader of field `SLP_SEL`"]
pub type SLP_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLP_SEL`"]
pub struct SLP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_SEL_W<'a> {
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
#[doc = "Reader of field `SLP_OE`"]
pub type SLP_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLP_OE`"]
pub struct SLP_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_OE_W<'a> {
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
    #[doc = "Bits 12:13 - configures IO_MUX function"]
    #[inline(always)]
    pub fn mcu_sel(&self) -> MCU_SEL_R {
        MCU_SEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - configures drive strength"]
    #[inline(always)]
    pub fn fun_drv(&self) -> FUN_DRV_R {
        FUN_DRV_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 9 - configures input enable"]
    #[inline(always)]
    pub fn fun_ie(&self) -> FUN_IE_R {
        FUN_IE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - configures pull up"]
    #[inline(always)]
    pub fn fun_pu(&self) -> FUN_PU_R {
        FUN_PU_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - configures pull down"]
    #[inline(always)]
    pub fn fun_pd(&self) -> FUN_PD_R {
        FUN_PD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - configures drive strength during sleep mode"]
    #[inline(always)]
    pub fn slp_drv(&self) -> SLP_DRV_R {
        SLP_DRV_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4 - configures input enable during sleep mode"]
    #[inline(always)]
    pub fn slp_ie(&self) -> SLP_IE_R {
        SLP_IE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - configures pull up during sleep mode"]
    #[inline(always)]
    pub fn slp_pu(&self) -> SLP_PU_R {
        SLP_PU_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - configures pull down during sleep mode"]
    #[inline(always)]
    pub fn slp_pd(&self) -> SLP_PD_R {
        SLP_PD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - configures sleep mode selection"]
    #[inline(always)]
    pub fn slp_sel(&self) -> SLP_SEL_R {
        SLP_SEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - configures output enable during sleep mode"]
    #[inline(always)]
    pub fn slp_oe(&self) -> SLP_OE_R {
        SLP_OE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 12:13 - configures IO_MUX function"]
    #[inline(always)]
    pub fn mcu_sel(&mut self) -> MCU_SEL_W {
        MCU_SEL_W { w: self }
    }
    #[doc = "Bits 10:11 - configures drive strength"]
    #[inline(always)]
    pub fn fun_drv(&mut self) -> FUN_DRV_W {
        FUN_DRV_W { w: self }
    }
    #[doc = "Bit 9 - configures input enable"]
    #[inline(always)]
    pub fn fun_ie(&mut self) -> FUN_IE_W {
        FUN_IE_W { w: self }
    }
    #[doc = "Bit 8 - configures pull up"]
    #[inline(always)]
    pub fn fun_pu(&mut self) -> FUN_PU_W {
        FUN_PU_W { w: self }
    }
    #[doc = "Bit 7 - configures pull down"]
    #[inline(always)]
    pub fn fun_pd(&mut self) -> FUN_PD_W {
        FUN_PD_W { w: self }
    }
    #[doc = "Bits 5:6 - configures drive strength during sleep mode"]
    #[inline(always)]
    pub fn slp_drv(&mut self) -> SLP_DRV_W {
        SLP_DRV_W { w: self }
    }
    #[doc = "Bit 4 - configures input enable during sleep mode"]
    #[inline(always)]
    pub fn slp_ie(&mut self) -> SLP_IE_W {
        SLP_IE_W { w: self }
    }
    #[doc = "Bit 3 - configures pull up during sleep mode"]
    #[inline(always)]
    pub fn slp_pu(&mut self) -> SLP_PU_W {
        SLP_PU_W { w: self }
    }
    #[doc = "Bit 2 - configures pull down during sleep mode"]
    #[inline(always)]
    pub fn slp_pd(&mut self) -> SLP_PD_W {
        SLP_PD_W { w: self }
    }
    #[doc = "Bit 1 - configures sleep mode selection"]
    #[inline(always)]
    pub fn slp_sel(&mut self) -> SLP_SEL_W {
        SLP_SEL_W { w: self }
    }
    #[doc = "Bit 0 - configures output enable during sleep mode"]
    #[inline(always)]
    pub fn slp_oe(&mut self) -> SLP_OE_W {
        SLP_OE_W { w: self }
    }
}
