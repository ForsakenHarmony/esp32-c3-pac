#[doc = "Reader of register RX_CONF1"]
pub type R = crate::R<u32, super::RX_CONF1>;
#[doc = "Writer for register RX_CONF1"]
pub type W = crate::W<u32, super::RX_CONF1>;
#[doc = "Register RX_CONF1 `reset()`'s with value 0"]
impl crate::ResetValue for super::RX_CONF1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RX_MSB_SHIFT`"]
pub type RX_MSB_SHIFT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_MSB_SHIFT`"]
pub struct RX_MSB_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_MSB_SHIFT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `RX_TDM_CHAN_BITS`"]
pub type RX_TDM_CHAN_BITS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_TDM_CHAN_BITS`"]
pub struct RX_TDM_CHAN_BITS_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TDM_CHAN_BITS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
#[doc = "Reader of field `RX_HALF_SAMPLE_BITS`"]
pub type RX_HALF_SAMPLE_BITS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_HALF_SAMPLE_BITS`"]
pub struct RX_HALF_SAMPLE_BITS_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_HALF_SAMPLE_BITS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
        self.w
    }
}
#[doc = "Reader of field `RX_BITS_MOD`"]
pub type RX_BITS_MOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_BITS_MOD`"]
pub struct RX_BITS_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_BITS_MOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 13)) | (((value as u32) & 0x1f) << 13);
        self.w
    }
}
#[doc = "Reader of field `RX_BCK_DIV_NUM`"]
pub type RX_BCK_DIV_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_BCK_DIV_NUM`"]
pub struct RX_BCK_DIV_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_BCK_DIV_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 7)) | (((value as u32) & 0x3f) << 7);
        self.w
    }
}
#[doc = "Reader of field `RX_TDM_WS_WIDTH`"]
pub type RX_TDM_WS_WIDTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_TDM_WS_WIDTH`"]
pub struct RX_TDM_WS_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TDM_WS_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rx_msb_shift(&self) -> RX_MSB_SHIFT_R {
        RX_MSB_SHIFT_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn rx_tdm_chan_bits(&self) -> RX_TDM_CHAN_BITS_R {
        RX_TDM_CHAN_BITS_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn rx_half_sample_bits(&self) -> RX_HALF_SAMPLE_BITS_R {
        RX_HALF_SAMPLE_BITS_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 13:17"]
    #[inline(always)]
    pub fn rx_bits_mod(&self) -> RX_BITS_MOD_R {
        RX_BITS_MOD_R::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bits 7:12"]
    #[inline(always)]
    pub fn rx_bck_div_num(&self) -> RX_BCK_DIV_NUM_R {
        RX_BCK_DIV_NUM_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn rx_tdm_ws_width(&self) -> RX_TDM_WS_WIDTH_R {
        RX_TDM_WS_WIDTH_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rx_msb_shift(&mut self) -> RX_MSB_SHIFT_W {
        RX_MSB_SHIFT_W { w: self }
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn rx_tdm_chan_bits(&mut self) -> RX_TDM_CHAN_BITS_W {
        RX_TDM_CHAN_BITS_W { w: self }
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn rx_half_sample_bits(&mut self) -> RX_HALF_SAMPLE_BITS_W {
        RX_HALF_SAMPLE_BITS_W { w: self }
    }
    #[doc = "Bits 13:17"]
    #[inline(always)]
    pub fn rx_bits_mod(&mut self) -> RX_BITS_MOD_W {
        RX_BITS_MOD_W { w: self }
    }
    #[doc = "Bits 7:12"]
    #[inline(always)]
    pub fn rx_bck_div_num(&mut self) -> RX_BCK_DIV_NUM_W {
        RX_BCK_DIV_NUM_W { w: self }
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn rx_tdm_ws_width(&mut self) -> RX_TDM_WS_WIDTH_W {
        RX_TDM_WS_WIDTH_W { w: self }
    }
}
