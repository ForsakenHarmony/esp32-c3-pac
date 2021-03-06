#[doc = "Reader of register TX_PCM2PDM_CONF"]
pub type R = crate::R<u32, super::TX_PCM2PDM_CONF>;
#[doc = "Writer for register TX_PCM2PDM_CONF"]
pub type W = crate::W<u32, super::TX_PCM2PDM_CONF>;
#[doc = "Register TX_PCM2PDM_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::TX_PCM2PDM_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PCM2PDM_CONV_EN`"]
pub type PCM2PDM_CONV_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCM2PDM_CONV_EN`"]
pub struct PCM2PDM_CONV_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PCM2PDM_CONV_EN_W<'a> {
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
#[doc = "Reader of field `TX_PDM_DAC_MODE_EN`"]
pub type TX_PDM_DAC_MODE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_PDM_DAC_MODE_EN`"]
pub struct TX_PDM_DAC_MODE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PDM_DAC_MODE_EN_W<'a> {
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
#[doc = "Reader of field `TX_PDM_DAC_2OUT_EN`"]
pub type TX_PDM_DAC_2OUT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_PDM_DAC_2OUT_EN`"]
pub struct TX_PDM_DAC_2OUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PDM_DAC_2OUT_EN_W<'a> {
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
#[doc = "Reader of field `TX_PDM_SIGMADELTA_DITHER`"]
pub type TX_PDM_SIGMADELTA_DITHER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_PDM_SIGMADELTA_DITHER`"]
pub struct TX_PDM_SIGMADELTA_DITHER_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PDM_SIGMADELTA_DITHER_W<'a> {
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
#[doc = "Reader of field `TX_PDM_SIGMADELTA_DITHER2`"]
pub type TX_PDM_SIGMADELTA_DITHER2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_PDM_SIGMADELTA_DITHER2`"]
pub struct TX_PDM_SIGMADELTA_DITHER2_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PDM_SIGMADELTA_DITHER2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `TX_PDM_SIGMADELTA_IN_SHIFT`"]
pub type TX_PDM_SIGMADELTA_IN_SHIFT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_PDM_SIGMADELTA_IN_SHIFT`"]
pub struct TX_PDM_SIGMADELTA_IN_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PDM_SIGMADELTA_IN_SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | (((value as u32) & 0x03) << 19);
        self.w
    }
}
#[doc = "Reader of field `TX_PDM_SINC_IN_SHIFT`"]
pub type TX_PDM_SINC_IN_SHIFT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_PDM_SINC_IN_SHIFT`"]
pub struct TX_PDM_SINC_IN_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PDM_SINC_IN_SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | (((value as u32) & 0x03) << 17);
        self.w
    }
}
#[doc = "Reader of field `TX_PDM_LP_IN_SHIFT`"]
pub type TX_PDM_LP_IN_SHIFT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_PDM_LP_IN_SHIFT`"]
pub struct TX_PDM_LP_IN_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PDM_LP_IN_SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 15)) | (((value as u32) & 0x03) << 15);
        self.w
    }
}
#[doc = "Reader of field `TX_PDM_HP_IN_SHIFT`"]
pub type TX_PDM_HP_IN_SHIFT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_PDM_HP_IN_SHIFT`"]
pub struct TX_PDM_HP_IN_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PDM_HP_IN_SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reader of field `TX_PDM_PRESCALE`"]
pub type TX_PDM_PRESCALE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_PDM_PRESCALE`"]
pub struct TX_PDM_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PDM_PRESCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 5)) | (((value as u32) & 0xff) << 5);
        self.w
    }
}
#[doc = "Reader of field `TX_PDM_SINC_OSR2`"]
pub type TX_PDM_SINC_OSR2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_PDM_SINC_OSR2`"]
pub struct TX_PDM_SINC_OSR2_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PDM_SINC_OSR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | (((value as u32) & 0x0f) << 1);
        self.w
    }
}
#[doc = "Reader of field `TX_PDM_HP_BYPASS`"]
pub type TX_PDM_HP_BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_PDM_HP_BYPASS`"]
pub struct TX_PDM_HP_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PDM_HP_BYPASS_W<'a> {
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
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pcm2pdm_conv_en(&self) -> PCM2PDM_CONV_EN_R {
        PCM2PDM_CONV_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tx_pdm_dac_mode_en(&self) -> TX_PDM_DAC_MODE_EN_R {
        TX_PDM_DAC_MODE_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn tx_pdm_dac_2out_en(&self) -> TX_PDM_DAC_2OUT_EN_R {
        TX_PDM_DAC_2OUT_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn tx_pdm_sigmadelta_dither(&self) -> TX_PDM_SIGMADELTA_DITHER_R {
        TX_PDM_SIGMADELTA_DITHER_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn tx_pdm_sigmadelta_dither2(&self) -> TX_PDM_SIGMADELTA_DITHER2_R {
        TX_PDM_SIGMADELTA_DITHER2_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    pub fn tx_pdm_sigmadelta_in_shift(&self) -> TX_PDM_SIGMADELTA_IN_SHIFT_R {
        TX_PDM_SIGMADELTA_IN_SHIFT_R::new(((self.bits >> 19) & 0x03) as u8)
    }
    #[doc = "Bits 17:18"]
    #[inline(always)]
    pub fn tx_pdm_sinc_in_shift(&self) -> TX_PDM_SINC_IN_SHIFT_R {
        TX_PDM_SINC_IN_SHIFT_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn tx_pdm_lp_in_shift(&self) -> TX_PDM_LP_IN_SHIFT_R {
        TX_PDM_LP_IN_SHIFT_R::new(((self.bits >> 15) & 0x03) as u8)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn tx_pdm_hp_in_shift(&self) -> TX_PDM_HP_IN_SHIFT_R {
        TX_PDM_HP_IN_SHIFT_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bits 5:12"]
    #[inline(always)]
    pub fn tx_pdm_prescale(&self) -> TX_PDM_PRESCALE_R {
        TX_PDM_PRESCALE_R::new(((self.bits >> 5) & 0xff) as u8)
    }
    #[doc = "Bits 1:4"]
    #[inline(always)]
    pub fn tx_pdm_sinc_osr2(&self) -> TX_PDM_SINC_OSR2_R {
        TX_PDM_SINC_OSR2_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_pdm_hp_bypass(&self) -> TX_PDM_HP_BYPASS_R {
        TX_PDM_HP_BYPASS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pcm2pdm_conv_en(&mut self) -> PCM2PDM_CONV_EN_W {
        PCM2PDM_CONV_EN_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tx_pdm_dac_mode_en(&mut self) -> TX_PDM_DAC_MODE_EN_W {
        TX_PDM_DAC_MODE_EN_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn tx_pdm_dac_2out_en(&mut self) -> TX_PDM_DAC_2OUT_EN_W {
        TX_PDM_DAC_2OUT_EN_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn tx_pdm_sigmadelta_dither(&mut self) -> TX_PDM_SIGMADELTA_DITHER_W {
        TX_PDM_SIGMADELTA_DITHER_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn tx_pdm_sigmadelta_dither2(&mut self) -> TX_PDM_SIGMADELTA_DITHER2_W {
        TX_PDM_SIGMADELTA_DITHER2_W { w: self }
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    pub fn tx_pdm_sigmadelta_in_shift(&mut self) -> TX_PDM_SIGMADELTA_IN_SHIFT_W {
        TX_PDM_SIGMADELTA_IN_SHIFT_W { w: self }
    }
    #[doc = "Bits 17:18"]
    #[inline(always)]
    pub fn tx_pdm_sinc_in_shift(&mut self) -> TX_PDM_SINC_IN_SHIFT_W {
        TX_PDM_SINC_IN_SHIFT_W { w: self }
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn tx_pdm_lp_in_shift(&mut self) -> TX_PDM_LP_IN_SHIFT_W {
        TX_PDM_LP_IN_SHIFT_W { w: self }
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn tx_pdm_hp_in_shift(&mut self) -> TX_PDM_HP_IN_SHIFT_W {
        TX_PDM_HP_IN_SHIFT_W { w: self }
    }
    #[doc = "Bits 5:12"]
    #[inline(always)]
    pub fn tx_pdm_prescale(&mut self) -> TX_PDM_PRESCALE_W {
        TX_PDM_PRESCALE_W { w: self }
    }
    #[doc = "Bits 1:4"]
    #[inline(always)]
    pub fn tx_pdm_sinc_osr2(&mut self) -> TX_PDM_SINC_OSR2_W {
        TX_PDM_SINC_OSR2_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_pdm_hp_bypass(&mut self) -> TX_PDM_HP_BYPASS_W {
        TX_PDM_HP_BYPASS_W { w: self }
    }
}
