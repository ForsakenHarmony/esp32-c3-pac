#[doc = "Reader of register TX_SIM"]
pub type R = crate::R<u32, super::TX_SIM>;
#[doc = "Writer for register TX_SIM"]
pub type W = crate::W<u32, super::TX_SIM>;
#[doc = "Register TX_SIM `reset()`'s with value 0"]
impl crate::ResetValue for super::TX_SIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TX_SIM_EN`"]
pub type TX_SIM_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_SIM_EN`"]
pub struct TX_SIM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_SIM_EN_W<'a> {
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
#[doc = "Reader of field `TX_SIM_CH1`"]
pub type TX_SIM_CH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_SIM_CH1`"]
pub struct TX_SIM_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_SIM_CH1_W<'a> {
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
#[doc = "Reader of field `TX_SIM_CH0`"]
pub type TX_SIM_CH0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_SIM_CH0`"]
pub struct TX_SIM_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_SIM_CH0_W<'a> {
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
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx_sim_en(&self) -> TX_SIM_EN_R {
        TX_SIM_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_sim_ch1(&self) -> TX_SIM_CH1_R {
        TX_SIM_CH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_sim_ch0(&self) -> TX_SIM_CH0_R {
        TX_SIM_CH0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx_sim_en(&mut self) -> TX_SIM_EN_W {
        TX_SIM_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_sim_ch1(&mut self) -> TX_SIM_CH1_W {
        TX_SIM_CH1_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_sim_ch0(&mut self) -> TX_SIM_CH0_W {
        TX_SIM_CH0_W { w: self }
    }
}
