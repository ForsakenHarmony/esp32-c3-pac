#[doc = "Reader of register CH1CONF0"]
pub type R = crate::R<u32, super::CH1CONF0>;
#[doc = "Writer for register CH1CONF0"]
pub type W = crate::W<u32, super::CH1CONF0>;
#[doc = "Register CH1CONF0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CH1CONF0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CONF_UPDATE_CH1`"]
pub struct CONF_UPDATE_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> CONF_UPDATE_CH1_W<'a> {
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
#[doc = "Write proxy for field `AFIFO_RST_CH1`"]
pub struct AFIFO_RST_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> AFIFO_RST_CH1_W<'a> {
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
#[doc = "Reader of field `CARRIER_OUT_LV_CH1`"]
pub type CARRIER_OUT_LV_CH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARRIER_OUT_LV_CH1`"]
pub struct CARRIER_OUT_LV_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_OUT_LV_CH1_W<'a> {
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
#[doc = "Reader of field `CARRIER_EN_CH1`"]
pub type CARRIER_EN_CH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARRIER_EN_CH1`"]
pub struct CARRIER_EN_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_EN_CH1_W<'a> {
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
#[doc = "Reader of field `CARRIER_EFF_EN_CH1`"]
pub type CARRIER_EFF_EN_CH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARRIER_EFF_EN_CH1`"]
pub struct CARRIER_EFF_EN_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_EFF_EN_CH1_W<'a> {
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
#[doc = "Reader of field `MEM_SIZE_CH1`"]
pub type MEM_SIZE_CH1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_SIZE_CH1`"]
pub struct MEM_SIZE_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_SIZE_CH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `DIV_CNT_CH1`"]
pub type DIV_CNT_CH1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIV_CNT_CH1`"]
pub struct DIV_CNT_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_CNT_CH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `TX_STOP_CH1`"]
pub type TX_STOP_CH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_STOP_CH1`"]
pub struct TX_STOP_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_STOP_CH1_W<'a> {
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
#[doc = "Reader of field `IDLE_OUT_EN_CH1`"]
pub type IDLE_OUT_EN_CH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDLE_OUT_EN_CH1`"]
pub struct IDLE_OUT_EN_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_OUT_EN_CH1_W<'a> {
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
#[doc = "Reader of field `IDLE_OUT_LV_CH1`"]
pub type IDLE_OUT_LV_CH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDLE_OUT_LV_CH1`"]
pub struct IDLE_OUT_LV_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_OUT_LV_CH1_W<'a> {
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
#[doc = "Reader of field `MEM_TX_WRAP_EN_CH1`"]
pub type MEM_TX_WRAP_EN_CH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MEM_TX_WRAP_EN_CH1`"]
pub struct MEM_TX_WRAP_EN_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_TX_WRAP_EN_CH1_W<'a> {
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
#[doc = "Reader of field `TX_CONTI_MODE_CH1`"]
pub type TX_CONTI_MODE_CH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_CONTI_MODE_CH1`"]
pub struct TX_CONTI_MODE_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CONTI_MODE_CH1_W<'a> {
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
#[doc = "Write proxy for field `APB_MEM_RST_CH1`"]
pub struct APB_MEM_RST_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_MEM_RST_CH1_W<'a> {
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
#[doc = "Write proxy for field `MEM_RD_RST_CH1`"]
pub struct MEM_RD_RST_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_RD_RST_CH1_W<'a> {
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
#[doc = "Write proxy for field `TX_START_CH1`"]
pub struct TX_START_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_START_CH1_W<'a> {
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
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn carrier_out_lv_ch1(&self) -> CARRIER_OUT_LV_CH1_R {
        CARRIER_OUT_LV_CH1_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn carrier_en_ch1(&self) -> CARRIER_EN_CH1_R {
        CARRIER_EN_CH1_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn carrier_eff_en_ch1(&self) -> CARRIER_EFF_EN_CH1_R {
        CARRIER_EFF_EN_CH1_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn mem_size_ch1(&self) -> MEM_SIZE_CH1_R {
        MEM_SIZE_CH1_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn div_cnt_ch1(&self) -> DIV_CNT_CH1_R {
        DIV_CNT_CH1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tx_stop_ch1(&self) -> TX_STOP_CH1_R {
        TX_STOP_CH1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn idle_out_en_ch1(&self) -> IDLE_OUT_EN_CH1_R {
        IDLE_OUT_EN_CH1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn idle_out_lv_ch1(&self) -> IDLE_OUT_LV_CH1_R {
        IDLE_OUT_LV_CH1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn mem_tx_wrap_en_ch1(&self) -> MEM_TX_WRAP_EN_CH1_R {
        MEM_TX_WRAP_EN_CH1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tx_conti_mode_ch1(&self) -> TX_CONTI_MODE_CH1_R {
        TX_CONTI_MODE_CH1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn conf_update_ch1(&mut self) -> CONF_UPDATE_CH1_W {
        CONF_UPDATE_CH1_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn afifo_rst_ch1(&mut self) -> AFIFO_RST_CH1_W {
        AFIFO_RST_CH1_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn carrier_out_lv_ch1(&mut self) -> CARRIER_OUT_LV_CH1_W {
        CARRIER_OUT_LV_CH1_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn carrier_en_ch1(&mut self) -> CARRIER_EN_CH1_W {
        CARRIER_EN_CH1_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn carrier_eff_en_ch1(&mut self) -> CARRIER_EFF_EN_CH1_W {
        CARRIER_EFF_EN_CH1_W { w: self }
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn mem_size_ch1(&mut self) -> MEM_SIZE_CH1_W {
        MEM_SIZE_CH1_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn div_cnt_ch1(&mut self) -> DIV_CNT_CH1_W {
        DIV_CNT_CH1_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tx_stop_ch1(&mut self) -> TX_STOP_CH1_W {
        TX_STOP_CH1_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn idle_out_en_ch1(&mut self) -> IDLE_OUT_EN_CH1_W {
        IDLE_OUT_EN_CH1_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn idle_out_lv_ch1(&mut self) -> IDLE_OUT_LV_CH1_W {
        IDLE_OUT_LV_CH1_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn mem_tx_wrap_en_ch1(&mut self) -> MEM_TX_WRAP_EN_CH1_W {
        MEM_TX_WRAP_EN_CH1_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tx_conti_mode_ch1(&mut self) -> TX_CONTI_MODE_CH1_W {
        TX_CONTI_MODE_CH1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn apb_mem_rst_ch1(&mut self) -> APB_MEM_RST_CH1_W {
        APB_MEM_RST_CH1_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn mem_rd_rst_ch1(&mut self) -> MEM_RD_RST_CH1_W {
        MEM_RD_RST_CH1_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_start_ch1(&mut self) -> TX_START_CH1_W {
        TX_START_CH1_W { w: self }
    }
}
