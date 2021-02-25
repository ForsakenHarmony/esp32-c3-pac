#[doc = "Reader of register RESET_STATE"]
pub type R = crate::R<u32, super::RESET_STATE>;
#[doc = "Writer for register RESET_STATE"]
pub type W = crate::W<u32, super::RESET_STATE>;
#[doc = "Register RESET_STATE `reset()`'s with value 0"]
impl crate::ResetValue for super::RESET_STATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DRESET_MASK_PROCPU`"]
pub type DRESET_MASK_PROCPU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DRESET_MASK_PROCPU`"]
pub struct DRESET_MASK_PROCPU_W<'a> {
    w: &'a mut W,
}
impl<'a> DRESET_MASK_PROCPU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `DRESET_MASK_APPCPU`"]
pub type DRESET_MASK_APPCPU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DRESET_MASK_APPCPU`"]
pub struct DRESET_MASK_APPCPU_W<'a> {
    w: &'a mut W,
}
impl<'a> DRESET_MASK_APPCPU_W<'a> {
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
#[doc = "Write proxy for field `JTAG_RESET_FLAG_CLR_APPCPU`"]
pub struct JTAG_RESET_FLAG_CLR_APPCPU_W<'a> {
    w: &'a mut W,
}
impl<'a> JTAG_RESET_FLAG_CLR_APPCPU_W<'a> {
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
#[doc = "Write proxy for field `JTAG_RESET_FLAG_CLR_PROCPU`"]
pub struct JTAG_RESET_FLAG_CLR_PROCPU_W<'a> {
    w: &'a mut W,
}
impl<'a> JTAG_RESET_FLAG_CLR_PROCPU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `JTAG_RESET_FLAG_APPCPU`"]
pub type JTAG_RESET_FLAG_APPCPU_R = crate::R<bool, bool>;
#[doc = "Reader of field `JTAG_RESET_FLAG_PROCPU`"]
pub type JTAG_RESET_FLAG_PROCPU_R = crate::R<bool, bool>;
#[doc = "Reader of field `OCD_HALT_ON_RESET_PROCPU`"]
pub type OCD_HALT_ON_RESET_PROCPU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OCD_HALT_ON_RESET_PROCPU`"]
pub struct OCD_HALT_ON_RESET_PROCPU_W<'a> {
    w: &'a mut W,
}
impl<'a> OCD_HALT_ON_RESET_PROCPU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `OCD_HALT_ON_RESET_APPCPU`"]
pub type OCD_HALT_ON_RESET_APPCPU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OCD_HALT_ON_RESET_APPCPU`"]
pub struct OCD_HALT_ON_RESET_APPCPU_W<'a> {
    w: &'a mut W,
}
impl<'a> OCD_HALT_ON_RESET_APPCPU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Write proxy for field `ALL_RESET_FLAG_CLR_APPCPU`"]
pub struct ALL_RESET_FLAG_CLR_APPCPU_W<'a> {
    w: &'a mut W,
}
impl<'a> ALL_RESET_FLAG_CLR_APPCPU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Write proxy for field `ALL_RESET_FLAG_CLR_PROCPU`"]
pub struct ALL_RESET_FLAG_CLR_PROCPU_W<'a> {
    w: &'a mut W,
}
impl<'a> ALL_RESET_FLAG_CLR_PROCPU_W<'a> {
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
#[doc = "Reader of field `ALL_RESET_FLAG_APPCPU`"]
pub type ALL_RESET_FLAG_APPCPU_R = crate::R<bool, bool>;
#[doc = "Reader of field `ALL_RESET_FLAG_PROCPU`"]
pub type ALL_RESET_FLAG_PROCPU_R = crate::R<bool, bool>;
#[doc = "Reader of field `STAT_VECTOR_SEL_PROCPU`"]
pub type STAT_VECTOR_SEL_PROCPU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STAT_VECTOR_SEL_PROCPU`"]
pub struct STAT_VECTOR_SEL_PROCPU_W<'a> {
    w: &'a mut W,
}
impl<'a> STAT_VECTOR_SEL_PROCPU_W<'a> {
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
#[doc = "Reader of field `STAT_VECTOR_SEL_APPCPU`"]
pub type STAT_VECTOR_SEL_APPCPU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STAT_VECTOR_SEL_APPCPU`"]
pub struct STAT_VECTOR_SEL_APPCPU_W<'a> {
    w: &'a mut W,
}
impl<'a> STAT_VECTOR_SEL_APPCPU_W<'a> {
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
#[doc = "Reader of field `RESET_CAUSE_APPCPU`"]
pub type RESET_CAUSE_APPCPU_R = crate::R<u8, u8>;
#[doc = "Reader of field `RESET_CAUSE_PROCPU`"]
pub type RESET_CAUSE_PROCPU_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dreset_mask_procpu(&self) -> DRESET_MASK_PROCPU_R {
        DRESET_MASK_PROCPU_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn dreset_mask_appcpu(&self) -> DRESET_MASK_APPCPU_R {
        DRESET_MASK_APPCPU_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn jtag_reset_flag_appcpu(&self) -> JTAG_RESET_FLAG_APPCPU_R {
        JTAG_RESET_FLAG_APPCPU_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn jtag_reset_flag_procpu(&self) -> JTAG_RESET_FLAG_PROCPU_R {
        JTAG_RESET_FLAG_PROCPU_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ocd_halt_on_reset_procpu(&self) -> OCD_HALT_ON_RESET_PROCPU_R {
        OCD_HALT_ON_RESET_PROCPU_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ocd_halt_on_reset_appcpu(&self) -> OCD_HALT_ON_RESET_APPCPU_R {
        OCD_HALT_ON_RESET_APPCPU_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn all_reset_flag_appcpu(&self) -> ALL_RESET_FLAG_APPCPU_R {
        ALL_RESET_FLAG_APPCPU_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn all_reset_flag_procpu(&self) -> ALL_RESET_FLAG_PROCPU_R {
        ALL_RESET_FLAG_PROCPU_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn stat_vector_sel_procpu(&self) -> STAT_VECTOR_SEL_PROCPU_R {
        STAT_VECTOR_SEL_PROCPU_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn stat_vector_sel_appcpu(&self) -> STAT_VECTOR_SEL_APPCPU_R {
        STAT_VECTOR_SEL_APPCPU_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn reset_cause_appcpu(&self) -> RESET_CAUSE_APPCPU_R {
        RESET_CAUSE_APPCPU_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn reset_cause_procpu(&self) -> RESET_CAUSE_PROCPU_R {
        RESET_CAUSE_PROCPU_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dreset_mask_procpu(&mut self) -> DRESET_MASK_PROCPU_W {
        DRESET_MASK_PROCPU_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn dreset_mask_appcpu(&mut self) -> DRESET_MASK_APPCPU_W {
        DRESET_MASK_APPCPU_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn jtag_reset_flag_clr_appcpu(&mut self) -> JTAG_RESET_FLAG_CLR_APPCPU_W {
        JTAG_RESET_FLAG_CLR_APPCPU_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn jtag_reset_flag_clr_procpu(&mut self) -> JTAG_RESET_FLAG_CLR_PROCPU_W {
        JTAG_RESET_FLAG_CLR_PROCPU_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ocd_halt_on_reset_procpu(&mut self) -> OCD_HALT_ON_RESET_PROCPU_W {
        OCD_HALT_ON_RESET_PROCPU_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ocd_halt_on_reset_appcpu(&mut self) -> OCD_HALT_ON_RESET_APPCPU_W {
        OCD_HALT_ON_RESET_APPCPU_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn all_reset_flag_clr_appcpu(&mut self) -> ALL_RESET_FLAG_CLR_APPCPU_W {
        ALL_RESET_FLAG_CLR_APPCPU_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn all_reset_flag_clr_procpu(&mut self) -> ALL_RESET_FLAG_CLR_PROCPU_W {
        ALL_RESET_FLAG_CLR_PROCPU_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn stat_vector_sel_procpu(&mut self) -> STAT_VECTOR_SEL_PROCPU_W {
        STAT_VECTOR_SEL_PROCPU_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn stat_vector_sel_appcpu(&mut self) -> STAT_VECTOR_SEL_APPCPU_W {
        STAT_VECTOR_SEL_APPCPU_W { w: self }
    }
}
