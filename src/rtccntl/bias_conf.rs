#[doc = "Reader of register BIAS_CONF"]
pub type R = crate::R<u32, super::BIAS_CONF>;
#[doc = "Writer for register BIAS_CONF"]
pub type W = crate::W<u32, super::BIAS_CONF>;
#[doc = "Register BIAS_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::BIAS_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DBG_ATTEN_MONITOR`"]
pub type DBG_ATTEN_MONITOR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DBG_ATTEN_MONITOR`"]
pub struct DBG_ATTEN_MONITOR_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_ATTEN_MONITOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 22)) | (((value as u32) & 0x0f) << 22);
        self.w
    }
}
#[doc = "Reader of field `DBG_ATTEN_DEEP_SLP`"]
pub type DBG_ATTEN_DEEP_SLP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DBG_ATTEN_DEEP_SLP`"]
pub struct DBG_ATTEN_DEEP_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_ATTEN_DEEP_SLP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | (((value as u32) & 0x0f) << 18);
        self.w
    }
}
#[doc = "Reader of field `BIAS_SLEEP_MONITOR`"]
pub type BIAS_SLEEP_MONITOR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BIAS_SLEEP_MONITOR`"]
pub struct BIAS_SLEEP_MONITOR_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_SLEEP_MONITOR_W<'a> {
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
#[doc = "Reader of field `BIAS_SLEEP_DEEP_SLP`"]
pub type BIAS_SLEEP_DEEP_SLP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BIAS_SLEEP_DEEP_SLP`"]
pub struct BIAS_SLEEP_DEEP_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_SLEEP_DEEP_SLP_W<'a> {
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
#[doc = "Reader of field `PD_CUR_MONITOR`"]
pub type PD_CUR_MONITOR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PD_CUR_MONITOR`"]
pub struct PD_CUR_MONITOR_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_CUR_MONITOR_W<'a> {
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
#[doc = "Reader of field `PD_CUR_DEEP_SLP`"]
pub type PD_CUR_DEEP_SLP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PD_CUR_DEEP_SLP`"]
pub struct PD_CUR_DEEP_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_CUR_DEEP_SLP_W<'a> {
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
#[doc = "Reader of field `BIAS_BUF_MONITOR`"]
pub type BIAS_BUF_MONITOR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BIAS_BUF_MONITOR`"]
pub struct BIAS_BUF_MONITOR_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_BUF_MONITOR_W<'a> {
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
#[doc = "Reader of field `BIAS_BUF_DEEP_SLP`"]
pub type BIAS_BUF_DEEP_SLP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BIAS_BUF_DEEP_SLP`"]
pub struct BIAS_BUF_DEEP_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_BUF_DEEP_SLP_W<'a> {
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
#[doc = "Reader of field `BIAS_BUF_WAKE`"]
pub type BIAS_BUF_WAKE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BIAS_BUF_WAKE`"]
pub struct BIAS_BUF_WAKE_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_BUF_WAKE_W<'a> {
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
#[doc = "Reader of field `BIAS_BUF_IDLE`"]
pub type BIAS_BUF_IDLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BIAS_BUF_IDLE`"]
pub struct BIAS_BUF_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_BUF_IDLE_W<'a> {
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
#[doc = "Reader of field `DG_VDD_DRV_B_SLP_EN`"]
pub type DG_VDD_DRV_B_SLP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DG_VDD_DRV_B_SLP_EN`"]
pub struct DG_VDD_DRV_B_SLP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_VDD_DRV_B_SLP_EN_W<'a> {
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
#[doc = "Reader of field `DG_VDD_DRV_B_SLP`"]
pub type DG_VDD_DRV_B_SLP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DG_VDD_DRV_B_SLP`"]
pub struct DG_VDD_DRV_B_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_VDD_DRV_B_SLP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn dbg_atten_monitor(&self) -> DBG_ATTEN_MONITOR_R {
        DBG_ATTEN_MONITOR_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn dbg_atten_deep_slp(&self) -> DBG_ATTEN_DEEP_SLP_R {
        DBG_ATTEN_DEEP_SLP_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn bias_sleep_monitor(&self) -> BIAS_SLEEP_MONITOR_R {
        BIAS_SLEEP_MONITOR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn bias_sleep_deep_slp(&self) -> BIAS_SLEEP_DEEP_SLP_R {
        BIAS_SLEEP_DEEP_SLP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pd_cur_monitor(&self) -> PD_CUR_MONITOR_R {
        PD_CUR_MONITOR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pd_cur_deep_slp(&self) -> PD_CUR_DEEP_SLP_R {
        PD_CUR_DEEP_SLP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn bias_buf_monitor(&self) -> BIAS_BUF_MONITOR_R {
        BIAS_BUF_MONITOR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn bias_buf_deep_slp(&self) -> BIAS_BUF_DEEP_SLP_R {
        BIAS_BUF_DEEP_SLP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn bias_buf_wake(&self) -> BIAS_BUF_WAKE_R {
        BIAS_BUF_WAKE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn bias_buf_idle(&self) -> BIAS_BUF_IDLE_R {
        BIAS_BUF_IDLE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dg_vdd_drv_b_slp_en(&self) -> DG_VDD_DRV_B_SLP_EN_R {
        DG_VDD_DRV_B_SLP_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dg_vdd_drv_b_slp(&self) -> DG_VDD_DRV_B_SLP_R {
        DG_VDD_DRV_B_SLP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn dbg_atten_monitor(&mut self) -> DBG_ATTEN_MONITOR_W {
        DBG_ATTEN_MONITOR_W { w: self }
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn dbg_atten_deep_slp(&mut self) -> DBG_ATTEN_DEEP_SLP_W {
        DBG_ATTEN_DEEP_SLP_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn bias_sleep_monitor(&mut self) -> BIAS_SLEEP_MONITOR_W {
        BIAS_SLEEP_MONITOR_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn bias_sleep_deep_slp(&mut self) -> BIAS_SLEEP_DEEP_SLP_W {
        BIAS_SLEEP_DEEP_SLP_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pd_cur_monitor(&mut self) -> PD_CUR_MONITOR_W {
        PD_CUR_MONITOR_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pd_cur_deep_slp(&mut self) -> PD_CUR_DEEP_SLP_W {
        PD_CUR_DEEP_SLP_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn bias_buf_monitor(&mut self) -> BIAS_BUF_MONITOR_W {
        BIAS_BUF_MONITOR_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn bias_buf_deep_slp(&mut self) -> BIAS_BUF_DEEP_SLP_W {
        BIAS_BUF_DEEP_SLP_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn bias_buf_wake(&mut self) -> BIAS_BUF_WAKE_W {
        BIAS_BUF_WAKE_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn bias_buf_idle(&mut self) -> BIAS_BUF_IDLE_W {
        BIAS_BUF_IDLE_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dg_vdd_drv_b_slp_en(&mut self) -> DG_VDD_DRV_B_SLP_EN_W {
        DG_VDD_DRV_B_SLP_EN_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dg_vdd_drv_b_slp(&mut self) -> DG_VDD_DRV_B_SLP_W {
        DG_VDD_DRV_B_SLP_W { w: self }
    }
}
