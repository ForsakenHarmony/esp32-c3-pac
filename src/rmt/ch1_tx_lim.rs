#[doc = "Reader of register CH1_TX_LIM"]
pub type R = crate::R<u32, super::CH1_TX_LIM>;
#[doc = "Writer for register CH1_TX_LIM"]
pub type W = crate::W<u32, super::CH1_TX_LIM>;
#[doc = "Register CH1_TX_LIM `reset()`'s with value 0"]
impl crate::ResetValue for super::CH1_TX_LIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `LOOP_COUNT_RESET_CH1`"]
pub struct LOOP_COUNT_RESET_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOP_COUNT_RESET_CH1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `TX_LOOP_CNT_EN_CH1`"]
pub type TX_LOOP_CNT_EN_CH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_LOOP_CNT_EN_CH1`"]
pub struct TX_LOOP_CNT_EN_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_LOOP_CNT_EN_CH1_W<'a> {
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
#[doc = "Reader of field `TX_LOOP_NUM_CH1`"]
pub type TX_LOOP_NUM_CH1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TX_LOOP_NUM_CH1`"]
pub struct TX_LOOP_NUM_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_LOOP_NUM_CH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 9)) | (((value as u32) & 0x03ff) << 9);
        self.w
    }
}
#[doc = "Reader of field `TX_LIM_CH1`"]
pub type TX_LIM_CH1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TX_LIM_CH1`"]
pub struct TX_LIM_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_LIM_CH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tx_loop_cnt_en_ch1(&self) -> TX_LOOP_CNT_EN_CH1_R {
        TX_LOOP_CNT_EN_CH1_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 9:18"]
    #[inline(always)]
    pub fn tx_loop_num_ch1(&self) -> TX_LOOP_NUM_CH1_R {
        TX_LOOP_NUM_CH1_R::new(((self.bits >> 9) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn tx_lim_ch1(&self) -> TX_LIM_CH1_R {
        TX_LIM_CH1_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn loop_count_reset_ch1(&mut self) -> LOOP_COUNT_RESET_CH1_W {
        LOOP_COUNT_RESET_CH1_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tx_loop_cnt_en_ch1(&mut self) -> TX_LOOP_CNT_EN_CH1_W {
        TX_LOOP_CNT_EN_CH1_W { w: self }
    }
    #[doc = "Bits 9:18"]
    #[inline(always)]
    pub fn tx_loop_num_ch1(&mut self) -> TX_LOOP_NUM_CH1_W {
        TX_LOOP_NUM_CH1_W { w: self }
    }
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn tx_lim_ch1(&mut self) -> TX_LIM_CH1_W {
        TX_LIM_CH1_W { w: self }
    }
}
