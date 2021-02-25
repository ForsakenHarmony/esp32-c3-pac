#[doc = "Reader of register RX_TDM_CTRL"]
pub type R = crate::R<u32, super::RX_TDM_CTRL>;
#[doc = "Writer for register RX_TDM_CTRL"]
pub type W = crate::W<u32, super::RX_TDM_CTRL>;
#[doc = "Register RX_TDM_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::RX_TDM_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RX_TDM_TOT_CHAN_NUM`"]
pub type RX_TDM_TOT_CHAN_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_TDM_TOT_CHAN_NUM`"]
pub struct RX_TDM_TOT_CHAN_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TDM_TOT_CHAN_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `RX_TDM_CHAN15_EN`"]
pub type RX_TDM_CHAN15_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_TDM_CHAN15_EN`"]
pub struct RX_TDM_CHAN15_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TDM_CHAN15_EN_W<'a> {
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
#[doc = "Reader of field `RX_TDM_CHAN14_EN`"]
pub type RX_TDM_CHAN14_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_TDM_CHAN14_EN`"]
pub struct RX_TDM_CHAN14_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TDM_CHAN14_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `RX_TDM_CHAN13_EN`"]
pub type RX_TDM_CHAN13_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_TDM_CHAN13_EN`"]
pub struct RX_TDM_CHAN13_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TDM_CHAN13_EN_W<'a> {
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
#[doc = "Reader of field `RX_TDM_CHAN12_EN`"]
pub type RX_TDM_CHAN12_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_TDM_CHAN12_EN`"]
pub struct RX_TDM_CHAN12_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TDM_CHAN12_EN_W<'a> {
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
#[doc = "Reader of field `RX_TDM_CHAN11_EN`"]
pub type RX_TDM_CHAN11_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_TDM_CHAN11_EN`"]
pub struct RX_TDM_CHAN11_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TDM_CHAN11_EN_W<'a> {
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
#[doc = "Reader of field `RX_TDM_CHAN10_EN`"]
pub type RX_TDM_CHAN10_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_TDM_CHAN10_EN`"]
pub struct RX_TDM_CHAN10_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TDM_CHAN10_EN_W<'a> {
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
#[doc = "Reader of field `RX_TDM_CHAN9_EN`"]
pub type RX_TDM_CHAN9_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_TDM_CHAN9_EN`"]
pub struct RX_TDM_CHAN9_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TDM_CHAN9_EN_W<'a> {
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
#[doc = "Reader of field `RX_TDM_CHAN8_EN`"]
pub type RX_TDM_CHAN8_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_TDM_CHAN8_EN`"]
pub struct RX_TDM_CHAN8_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TDM_CHAN8_EN_W<'a> {
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
#[doc = "Reader of field `RX_TDM_PDM_CHAN7_EN`"]
pub type RX_TDM_PDM_CHAN7_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_TDM_PDM_CHAN7_EN`"]
pub struct RX_TDM_PDM_CHAN7_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TDM_PDM_CHAN7_EN_W<'a> {
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
#[doc = "Reader of field `RX_TDM_PDM_CHAN6_EN`"]
pub type RX_TDM_PDM_CHAN6_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_TDM_PDM_CHAN6_EN`"]
pub struct RX_TDM_PDM_CHAN6_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TDM_PDM_CHAN6_EN_W<'a> {
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
#[doc = "Reader of field `RX_TDM_PDM_CHAN5_EN`"]
pub type RX_TDM_PDM_CHAN5_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_TDM_PDM_CHAN5_EN`"]
pub struct RX_TDM_PDM_CHAN5_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TDM_PDM_CHAN5_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `RX_TDM_PDM_CHAN4_EN`"]
pub type RX_TDM_PDM_CHAN4_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_TDM_PDM_CHAN4_EN`"]
pub struct RX_TDM_PDM_CHAN4_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TDM_PDM_CHAN4_EN_W<'a> {
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
#[doc = "Reader of field `RX_TDM_PDM_CHAN3_EN`"]
pub type RX_TDM_PDM_CHAN3_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_TDM_PDM_CHAN3_EN`"]
pub struct RX_TDM_PDM_CHAN3_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TDM_PDM_CHAN3_EN_W<'a> {
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
#[doc = "Reader of field `RX_TDM_PDM_CHAN2_EN`"]
pub type RX_TDM_PDM_CHAN2_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_TDM_PDM_CHAN2_EN`"]
pub struct RX_TDM_PDM_CHAN2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TDM_PDM_CHAN2_EN_W<'a> {
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
#[doc = "Reader of field `RX_TDM_PDM_CHAN1_EN`"]
pub type RX_TDM_PDM_CHAN1_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_TDM_PDM_CHAN1_EN`"]
pub struct RX_TDM_PDM_CHAN1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TDM_PDM_CHAN1_EN_W<'a> {
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
#[doc = "Reader of field `RX_TDM_PDM_CHAN0_EN`"]
pub type RX_TDM_PDM_CHAN0_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_TDM_PDM_CHAN0_EN`"]
pub struct RX_TDM_PDM_CHAN0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TDM_PDM_CHAN0_EN_W<'a> {
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
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn rx_tdm_tot_chan_num(&self) -> RX_TDM_TOT_CHAN_NUM_R {
        RX_TDM_TOT_CHAN_NUM_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rx_tdm_chan15_en(&self) -> RX_TDM_CHAN15_EN_R {
        RX_TDM_CHAN15_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rx_tdm_chan14_en(&self) -> RX_TDM_CHAN14_EN_R {
        RX_TDM_CHAN14_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rx_tdm_chan13_en(&self) -> RX_TDM_CHAN13_EN_R {
        RX_TDM_CHAN13_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rx_tdm_chan12_en(&self) -> RX_TDM_CHAN12_EN_R {
        RX_TDM_CHAN12_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rx_tdm_chan11_en(&self) -> RX_TDM_CHAN11_EN_R {
        RX_TDM_CHAN11_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rx_tdm_chan10_en(&self) -> RX_TDM_CHAN10_EN_R {
        RX_TDM_CHAN10_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rx_tdm_chan9_en(&self) -> RX_TDM_CHAN9_EN_R {
        RX_TDM_CHAN9_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rx_tdm_chan8_en(&self) -> RX_TDM_CHAN8_EN_R {
        RX_TDM_CHAN8_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rx_tdm_pdm_chan7_en(&self) -> RX_TDM_PDM_CHAN7_EN_R {
        RX_TDM_PDM_CHAN7_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rx_tdm_pdm_chan6_en(&self) -> RX_TDM_PDM_CHAN6_EN_R {
        RX_TDM_PDM_CHAN6_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rx_tdm_pdm_chan5_en(&self) -> RX_TDM_PDM_CHAN5_EN_R {
        RX_TDM_PDM_CHAN5_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rx_tdm_pdm_chan4_en(&self) -> RX_TDM_PDM_CHAN4_EN_R {
        RX_TDM_PDM_CHAN4_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_tdm_pdm_chan3_en(&self) -> RX_TDM_PDM_CHAN3_EN_R {
        RX_TDM_PDM_CHAN3_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_tdm_pdm_chan2_en(&self) -> RX_TDM_PDM_CHAN2_EN_R {
        RX_TDM_PDM_CHAN2_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rx_tdm_pdm_chan1_en(&self) -> RX_TDM_PDM_CHAN1_EN_R {
        RX_TDM_PDM_CHAN1_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_tdm_pdm_chan0_en(&self) -> RX_TDM_PDM_CHAN0_EN_R {
        RX_TDM_PDM_CHAN0_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn rx_tdm_tot_chan_num(&mut self) -> RX_TDM_TOT_CHAN_NUM_W {
        RX_TDM_TOT_CHAN_NUM_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rx_tdm_chan15_en(&mut self) -> RX_TDM_CHAN15_EN_W {
        RX_TDM_CHAN15_EN_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rx_tdm_chan14_en(&mut self) -> RX_TDM_CHAN14_EN_W {
        RX_TDM_CHAN14_EN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rx_tdm_chan13_en(&mut self) -> RX_TDM_CHAN13_EN_W {
        RX_TDM_CHAN13_EN_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rx_tdm_chan12_en(&mut self) -> RX_TDM_CHAN12_EN_W {
        RX_TDM_CHAN12_EN_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rx_tdm_chan11_en(&mut self) -> RX_TDM_CHAN11_EN_W {
        RX_TDM_CHAN11_EN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rx_tdm_chan10_en(&mut self) -> RX_TDM_CHAN10_EN_W {
        RX_TDM_CHAN10_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rx_tdm_chan9_en(&mut self) -> RX_TDM_CHAN9_EN_W {
        RX_TDM_CHAN9_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rx_tdm_chan8_en(&mut self) -> RX_TDM_CHAN8_EN_W {
        RX_TDM_CHAN8_EN_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rx_tdm_pdm_chan7_en(&mut self) -> RX_TDM_PDM_CHAN7_EN_W {
        RX_TDM_PDM_CHAN7_EN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rx_tdm_pdm_chan6_en(&mut self) -> RX_TDM_PDM_CHAN6_EN_W {
        RX_TDM_PDM_CHAN6_EN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rx_tdm_pdm_chan5_en(&mut self) -> RX_TDM_PDM_CHAN5_EN_W {
        RX_TDM_PDM_CHAN5_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rx_tdm_pdm_chan4_en(&mut self) -> RX_TDM_PDM_CHAN4_EN_W {
        RX_TDM_PDM_CHAN4_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_tdm_pdm_chan3_en(&mut self) -> RX_TDM_PDM_CHAN3_EN_W {
        RX_TDM_PDM_CHAN3_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_tdm_pdm_chan2_en(&mut self) -> RX_TDM_PDM_CHAN2_EN_W {
        RX_TDM_PDM_CHAN2_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rx_tdm_pdm_chan1_en(&mut self) -> RX_TDM_PDM_CHAN1_EN_W {
        RX_TDM_PDM_CHAN1_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_tdm_pdm_chan0_en(&mut self) -> RX_TDM_PDM_CHAN0_EN_W {
        RX_TDM_PDM_CHAN0_EN_W { w: self }
    }
}
