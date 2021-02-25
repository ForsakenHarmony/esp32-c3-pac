#[doc = "Reader of register RX_CONF"]
pub type R = crate::R<u32, super::RX_CONF>;
#[doc = "Writer for register RX_CONF"]
pub type W = crate::W<u32, super::RX_CONF>;
#[doc = "Register RX_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::RX_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RX_PDM_EN`"]
pub type RX_PDM_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_PDM_EN`"]
pub struct RX_PDM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PDM_EN_W<'a> {
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
#[doc = "Reader of field `RX_TDM_EN`"]
pub type RX_TDM_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_TDM_EN`"]
pub struct RX_TDM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TDM_EN_W<'a> {
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
#[doc = "Reader of field `RX_BIT_ORDER`"]
pub type RX_BIT_ORDER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_BIT_ORDER`"]
pub struct RX_BIT_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_BIT_ORDER_W<'a> {
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
#[doc = "Reader of field `RX_WS_IDLE_POL`"]
pub type RX_WS_IDLE_POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_WS_IDLE_POL`"]
pub struct RX_WS_IDLE_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_WS_IDLE_POL_W<'a> {
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
#[doc = "Reader of field `RX_24_FILL_EN`"]
pub type RX_24_FILL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_24_FILL_EN`"]
pub struct RX_24_FILL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_24_FILL_EN_W<'a> {
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
#[doc = "Reader of field `RX_LEFT_ALIGN`"]
pub type RX_LEFT_ALIGN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_LEFT_ALIGN`"]
pub struct RX_LEFT_ALIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_LEFT_ALIGN_W<'a> {
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
#[doc = "Reader of field `RX_STOP_MODE`"]
pub type RX_STOP_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_STOP_MODE`"]
pub struct RX_STOP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_STOP_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reader of field `RX_PCM_BYPASS`"]
pub type RX_PCM_BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_PCM_BYPASS`"]
pub struct RX_PCM_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PCM_BYPASS_W<'a> {
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
#[doc = "Reader of field `RX_PCM_CONF`"]
pub type RX_PCM_CONF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_PCM_CONF`"]
pub struct RX_PCM_CONF_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PCM_CONF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `RX_MONO_FST_VLD`"]
pub type RX_MONO_FST_VLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_MONO_FST_VLD`"]
pub struct RX_MONO_FST_VLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_MONO_FST_VLD_W<'a> {
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
#[doc = "Reader of field `RX_UPDATE`"]
pub type RX_UPDATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_UPDATE`"]
pub struct RX_UPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_UPDATE_W<'a> {
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
#[doc = "Reader of field `RX_BIG_ENDIAN`"]
pub type RX_BIG_ENDIAN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_BIG_ENDIAN`"]
pub struct RX_BIG_ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_BIG_ENDIAN_W<'a> {
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
#[doc = "Reader of field `RX_MONO`"]
pub type RX_MONO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_MONO`"]
pub struct RX_MONO_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_MONO_W<'a> {
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
#[doc = "Reader of field `RX_SLAVE_MOD`"]
pub type RX_SLAVE_MOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_SLAVE_MOD`"]
pub struct RX_SLAVE_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_SLAVE_MOD_W<'a> {
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
#[doc = "Reader of field `RX_START`"]
pub type RX_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_START`"]
pub struct RX_START_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_START_W<'a> {
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
#[doc = "Write proxy for field `RX_FIFO_RESET`"]
pub struct RX_FIFO_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_RESET_W<'a> {
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
#[doc = "Write proxy for field `RX_RESET`"]
pub struct RX_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_RESET_W<'a> {
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
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rx_pdm_en(&self) -> RX_PDM_EN_R {
        RX_PDM_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rx_tdm_en(&self) -> RX_TDM_EN_R {
        RX_TDM_EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rx_bit_order(&self) -> RX_BIT_ORDER_R {
        RX_BIT_ORDER_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rx_ws_idle_pol(&self) -> RX_WS_IDLE_POL_R {
        RX_WS_IDLE_POL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rx_24_fill_en(&self) -> RX_24_FILL_EN_R {
        RX_24_FILL_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rx_left_align(&self) -> RX_LEFT_ALIGN_R {
        RX_LEFT_ALIGN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn rx_stop_mode(&self) -> RX_STOP_MODE_R {
        RX_STOP_MODE_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rx_pcm_bypass(&self) -> RX_PCM_BYPASS_R {
        RX_PCM_BYPASS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn rx_pcm_conf(&self) -> RX_PCM_CONF_R {
        RX_PCM_CONF_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rx_mono_fst_vld(&self) -> RX_MONO_FST_VLD_R {
        RX_MONO_FST_VLD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rx_update(&self) -> RX_UPDATE_R {
        RX_UPDATE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rx_big_endian(&self) -> RX_BIG_ENDIAN_R {
        RX_BIG_ENDIAN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rx_mono(&self) -> RX_MONO_R {
        RX_MONO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_slave_mod(&self) -> RX_SLAVE_MOD_R {
        RX_SLAVE_MOD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_start(&self) -> RX_START_R {
        RX_START_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rx_pdm_en(&mut self) -> RX_PDM_EN_W {
        RX_PDM_EN_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rx_tdm_en(&mut self) -> RX_TDM_EN_W {
        RX_TDM_EN_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rx_bit_order(&mut self) -> RX_BIT_ORDER_W {
        RX_BIT_ORDER_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rx_ws_idle_pol(&mut self) -> RX_WS_IDLE_POL_W {
        RX_WS_IDLE_POL_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rx_24_fill_en(&mut self) -> RX_24_FILL_EN_W {
        RX_24_FILL_EN_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rx_left_align(&mut self) -> RX_LEFT_ALIGN_W {
        RX_LEFT_ALIGN_W { w: self }
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn rx_stop_mode(&mut self) -> RX_STOP_MODE_W {
        RX_STOP_MODE_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rx_pcm_bypass(&mut self) -> RX_PCM_BYPASS_W {
        RX_PCM_BYPASS_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn rx_pcm_conf(&mut self) -> RX_PCM_CONF_W {
        RX_PCM_CONF_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rx_mono_fst_vld(&mut self) -> RX_MONO_FST_VLD_W {
        RX_MONO_FST_VLD_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rx_update(&mut self) -> RX_UPDATE_W {
        RX_UPDATE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rx_big_endian(&mut self) -> RX_BIG_ENDIAN_W {
        RX_BIG_ENDIAN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rx_mono(&mut self) -> RX_MONO_W {
        RX_MONO_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_slave_mod(&mut self) -> RX_SLAVE_MOD_W {
        RX_SLAVE_MOD_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_start(&mut self) -> RX_START_W {
        RX_START_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rx_fifo_reset(&mut self) -> RX_FIFO_RESET_W {
        RX_FIFO_RESET_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_reset(&mut self) -> RX_RESET_W {
        RX_RESET_W { w: self }
    }
}
