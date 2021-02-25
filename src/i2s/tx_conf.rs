#[doc = "Reader of register TX_CONF"]
pub type R = crate::R<u32, super::TX_CONF>;
#[doc = "Writer for register TX_CONF"]
pub type W = crate::W<u32, super::TX_CONF>;
#[doc = "Register TX_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::TX_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SIG_LOOPBACK`"]
pub type SIG_LOOPBACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIG_LOOPBACK`"]
pub struct SIG_LOOPBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> SIG_LOOPBACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `TX_CHAN_MOD`"]
pub type TX_CHAN_MOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_CHAN_MOD`"]
pub struct TX_CHAN_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CHAN_MOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `TX_PDM_EN`"]
pub type TX_PDM_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_PDM_EN`"]
pub struct TX_PDM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PDM_EN_W<'a> {
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
#[doc = "Reader of field `TX_TDM_EN`"]
pub type TX_TDM_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_TDM_EN`"]
pub struct TX_TDM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_TDM_EN_W<'a> {
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
#[doc = "Reader of field `TX_BIT_ORDER`"]
pub type TX_BIT_ORDER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_BIT_ORDER`"]
pub struct TX_BIT_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BIT_ORDER_W<'a> {
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
#[doc = "Reader of field `TX_WS_IDLE_POL`"]
pub type TX_WS_IDLE_POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_WS_IDLE_POL`"]
pub struct TX_WS_IDLE_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_WS_IDLE_POL_W<'a> {
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
#[doc = "Reader of field `TX_24_FILL_EN`"]
pub type TX_24_FILL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_24_FILL_EN`"]
pub struct TX_24_FILL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_24_FILL_EN_W<'a> {
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
#[doc = "Reader of field `TX_LEFT_ALIGN`"]
pub type TX_LEFT_ALIGN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_LEFT_ALIGN`"]
pub struct TX_LEFT_ALIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_LEFT_ALIGN_W<'a> {
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
#[doc = "Reader of field `TX_STOP_EN`"]
pub type TX_STOP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_STOP_EN`"]
pub struct TX_STOP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_STOP_EN_W<'a> {
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
#[doc = "Reader of field `TX_PCM_BYPASS`"]
pub type TX_PCM_BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_PCM_BYPASS`"]
pub struct TX_PCM_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PCM_BYPASS_W<'a> {
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
#[doc = "Reader of field `TX_PCM_CONF`"]
pub type TX_PCM_CONF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_PCM_CONF`"]
pub struct TX_PCM_CONF_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PCM_CONF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `TX_MONO_FST_VLD`"]
pub type TX_MONO_FST_VLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_MONO_FST_VLD`"]
pub struct TX_MONO_FST_VLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_MONO_FST_VLD_W<'a> {
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
#[doc = "Reader of field `TX_UPDATE`"]
pub type TX_UPDATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_UPDATE`"]
pub struct TX_UPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_UPDATE_W<'a> {
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
#[doc = "Reader of field `TX_BIG_ENDIAN`"]
pub type TX_BIG_ENDIAN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_BIG_ENDIAN`"]
pub struct TX_BIG_ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BIG_ENDIAN_W<'a> {
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
#[doc = "Reader of field `TX_CHAN_EQUAL`"]
pub type TX_CHAN_EQUAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_CHAN_EQUAL`"]
pub struct TX_CHAN_EQUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CHAN_EQUAL_W<'a> {
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
#[doc = "Reader of field `TX_MONO`"]
pub type TX_MONO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_MONO`"]
pub struct TX_MONO_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_MONO_W<'a> {
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
#[doc = "Reader of field `TX_SLAVE_MOD`"]
pub type TX_SLAVE_MOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_SLAVE_MOD`"]
pub struct TX_SLAVE_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_SLAVE_MOD_W<'a> {
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
#[doc = "Reader of field `TX_START`"]
pub type TX_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_START`"]
pub struct TX_START_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_START_W<'a> {
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
#[doc = "Write proxy for field `TX_FIFO_RESET`"]
pub struct TX_FIFO_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_RESET_W<'a> {
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
#[doc = "Write proxy for field `TX_RESET`"]
pub struct TX_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_RESET_W<'a> {
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
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sig_loopback(&self) -> SIG_LOOPBACK_R {
        SIG_LOOPBACK_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn tx_chan_mod(&self) -> TX_CHAN_MOD_R {
        TX_CHAN_MOD_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tx_pdm_en(&self) -> TX_PDM_EN_R {
        TX_PDM_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tx_tdm_en(&self) -> TX_TDM_EN_R {
        TX_TDM_EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tx_bit_order(&self) -> TX_BIT_ORDER_R {
        TX_BIT_ORDER_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tx_ws_idle_pol(&self) -> TX_WS_IDLE_POL_R {
        TX_WS_IDLE_POL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tx_24_fill_en(&self) -> TX_24_FILL_EN_R {
        TX_24_FILL_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tx_left_align(&self) -> TX_LEFT_ALIGN_R {
        TX_LEFT_ALIGN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tx_stop_en(&self) -> TX_STOP_EN_R {
        TX_STOP_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tx_pcm_bypass(&self) -> TX_PCM_BYPASS_R {
        TX_PCM_BYPASS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn tx_pcm_conf(&self) -> TX_PCM_CONF_R {
        TX_PCM_CONF_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tx_mono_fst_vld(&self) -> TX_MONO_FST_VLD_R {
        TX_MONO_FST_VLD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tx_update(&self) -> TX_UPDATE_R {
        TX_UPDATE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tx_big_endian(&self) -> TX_BIG_ENDIAN_R {
        TX_BIG_ENDIAN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tx_chan_equal(&self) -> TX_CHAN_EQUAL_R {
        TX_CHAN_EQUAL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tx_mono(&self) -> TX_MONO_R {
        TX_MONO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tx_slave_mod(&self) -> TX_SLAVE_MOD_R {
        TX_SLAVE_MOD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx_start(&self) -> TX_START_R {
        TX_START_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sig_loopback(&mut self) -> SIG_LOOPBACK_W {
        SIG_LOOPBACK_W { w: self }
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn tx_chan_mod(&mut self) -> TX_CHAN_MOD_W {
        TX_CHAN_MOD_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tx_pdm_en(&mut self) -> TX_PDM_EN_W {
        TX_PDM_EN_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tx_tdm_en(&mut self) -> TX_TDM_EN_W {
        TX_TDM_EN_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tx_bit_order(&mut self) -> TX_BIT_ORDER_W {
        TX_BIT_ORDER_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tx_ws_idle_pol(&mut self) -> TX_WS_IDLE_POL_W {
        TX_WS_IDLE_POL_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tx_24_fill_en(&mut self) -> TX_24_FILL_EN_W {
        TX_24_FILL_EN_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tx_left_align(&mut self) -> TX_LEFT_ALIGN_W {
        TX_LEFT_ALIGN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tx_stop_en(&mut self) -> TX_STOP_EN_W {
        TX_STOP_EN_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tx_pcm_bypass(&mut self) -> TX_PCM_BYPASS_W {
        TX_PCM_BYPASS_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn tx_pcm_conf(&mut self) -> TX_PCM_CONF_W {
        TX_PCM_CONF_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tx_mono_fst_vld(&mut self) -> TX_MONO_FST_VLD_W {
        TX_MONO_FST_VLD_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tx_update(&mut self) -> TX_UPDATE_W {
        TX_UPDATE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tx_big_endian(&mut self) -> TX_BIG_ENDIAN_W {
        TX_BIG_ENDIAN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tx_chan_equal(&mut self) -> TX_CHAN_EQUAL_W {
        TX_CHAN_EQUAL_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tx_mono(&mut self) -> TX_MONO_W {
        TX_MONO_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tx_slave_mod(&mut self) -> TX_SLAVE_MOD_W {
        TX_SLAVE_MOD_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx_start(&mut self) -> TX_START_W {
        TX_START_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_fifo_reset(&mut self) -> TX_FIFO_RESET_W {
        TX_FIFO_RESET_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_reset(&mut self) -> TX_RESET_W {
        TX_RESET_W { w: self }
    }
}
