#[doc = "Reader of register LSCH5_CONF0"]
pub type R = crate::R<u32, super::LSCH5_CONF0>;
#[doc = "Writer for register LSCH5_CONF0"]
pub type W = crate::W<u32, super::LSCH5_CONF0>;
#[doc = "Register LSCH5_CONF0 `reset()`'s with value 0"]
impl crate::ResetValue for super::LSCH5_CONF0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `OVF_CNT_RESET_LSCH5`"]
pub struct OVF_CNT_RESET_LSCH5_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_CNT_RESET_LSCH5_W<'a> {
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
#[doc = "Reader of field `OVF_CNT_EN_LSCH5`"]
pub type OVF_CNT_EN_LSCH5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVF_CNT_EN_LSCH5`"]
pub struct OVF_CNT_EN_LSCH5_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_CNT_EN_LSCH5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `OVF_NUM_LSCH5`"]
pub type OVF_NUM_LSCH5_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `OVF_NUM_LSCH5`"]
pub struct OVF_NUM_LSCH5_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_NUM_LSCH5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 5)) | (((value as u32) & 0x03ff) << 5);
        self.w
    }
}
#[doc = "Write proxy for field `PARA_UP_LSCH5`"]
pub struct PARA_UP_LSCH5_W<'a> {
    w: &'a mut W,
}
impl<'a> PARA_UP_LSCH5_W<'a> {
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
#[doc = "Reader of field `IDLE_LV_LSCH5`"]
pub type IDLE_LV_LSCH5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDLE_LV_LSCH5`"]
pub struct IDLE_LV_LSCH5_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_LV_LSCH5_W<'a> {
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
#[doc = "Reader of field `SIG_OUT_EN_LSCH5`"]
pub type SIG_OUT_EN_LSCH5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIG_OUT_EN_LSCH5`"]
pub struct SIG_OUT_EN_LSCH5_W<'a> {
    w: &'a mut W,
}
impl<'a> SIG_OUT_EN_LSCH5_W<'a> {
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
#[doc = "Reader of field `TIMER_SEL_LSCH5`"]
pub type TIMER_SEL_LSCH5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TIMER_SEL_LSCH5`"]
pub struct TIMER_SEL_LSCH5_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_SEL_LSCH5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ovf_cnt_en_lsch5(&self) -> OVF_CNT_EN_LSCH5_R {
        OVF_CNT_EN_LSCH5_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 5:14"]
    #[inline(always)]
    pub fn ovf_num_lsch5(&self) -> OVF_NUM_LSCH5_R {
        OVF_NUM_LSCH5_R::new(((self.bits >> 5) & 0x03ff) as u16)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn idle_lv_lsch5(&self) -> IDLE_LV_LSCH5_R {
        IDLE_LV_LSCH5_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sig_out_en_lsch5(&self) -> SIG_OUT_EN_LSCH5_R {
        SIG_OUT_EN_LSCH5_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn timer_sel_lsch5(&self) -> TIMER_SEL_LSCH5_R {
        TIMER_SEL_LSCH5_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ovf_cnt_reset_lsch5(&mut self) -> OVF_CNT_RESET_LSCH5_W {
        OVF_CNT_RESET_LSCH5_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ovf_cnt_en_lsch5(&mut self) -> OVF_CNT_EN_LSCH5_W {
        OVF_CNT_EN_LSCH5_W { w: self }
    }
    #[doc = "Bits 5:14"]
    #[inline(always)]
    pub fn ovf_num_lsch5(&mut self) -> OVF_NUM_LSCH5_W {
        OVF_NUM_LSCH5_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn para_up_lsch5(&mut self) -> PARA_UP_LSCH5_W {
        PARA_UP_LSCH5_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn idle_lv_lsch5(&mut self) -> IDLE_LV_LSCH5_W {
        IDLE_LV_LSCH5_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sig_out_en_lsch5(&mut self) -> SIG_OUT_EN_LSCH5_W {
        SIG_OUT_EN_LSCH5_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn timer_sel_lsch5(&mut self) -> TIMER_SEL_LSCH5_W {
        TIMER_SEL_LSCH5_W { w: self }
    }
}
