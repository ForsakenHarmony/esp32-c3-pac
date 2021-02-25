#[doc = "Reader of register SDIO_CONF"]
pub type R = crate::R<u32, super::SDIO_CONF>;
#[doc = "Writer for register SDIO_CONF"]
pub type W = crate::W<u32, super::SDIO_CONF>;
#[doc = "Register SDIO_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::SDIO_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XPD_SDIO_REG`"]
pub type XPD_SDIO_REG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XPD_SDIO_REG`"]
pub struct XPD_SDIO_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> XPD_SDIO_REG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `DREFH_SDIO`"]
pub type DREFH_SDIO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DREFH_SDIO`"]
pub struct DREFH_SDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> DREFH_SDIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
#[doc = "Reader of field `DREFM_SDIO`"]
pub type DREFM_SDIO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DREFM_SDIO`"]
pub struct DREFM_SDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> DREFM_SDIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
        self.w
    }
}
#[doc = "Reader of field `DREFL_SDIO`"]
pub type DREFL_SDIO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DREFL_SDIO`"]
pub struct DREFL_SDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> DREFL_SDIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
#[doc = "Reader of field `REG1P8_READY`"]
pub type REG1P8_READY_R = crate::R<bool, bool>;
#[doc = "Reader of field `SDIO_TIEH`"]
pub type SDIO_TIEH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDIO_TIEH`"]
pub struct SDIO_TIEH_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_TIEH_W<'a> {
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
#[doc = "Reader of field `SDIO_FORCE`"]
pub type SDIO_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDIO_FORCE`"]
pub struct SDIO_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_FORCE_W<'a> {
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
#[doc = "Reader of field `SDIO_PD_EN`"]
pub type SDIO_PD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDIO_PD_EN`"]
pub struct SDIO_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_PD_EN_W<'a> {
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
#[doc = "Reader of field `SDIO_ENCURLIM`"]
pub type SDIO_ENCURLIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDIO_ENCURLIM`"]
pub struct SDIO_ENCURLIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_ENCURLIM_W<'a> {
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
#[doc = "Reader of field `SDIO_MODECURLIM`"]
pub type SDIO_MODECURLIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDIO_MODECURLIM`"]
pub struct SDIO_MODECURLIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_MODECURLIM_W<'a> {
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
#[doc = "Reader of field `SDIO_DCURLIM`"]
pub type SDIO_DCURLIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDIO_DCURLIM`"]
pub struct SDIO_DCURLIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_DCURLIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `SDIO_EN_INITI`"]
pub type SDIO_EN_INITI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDIO_EN_INITI`"]
pub struct SDIO_EN_INITI_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_EN_INITI_W<'a> {
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
#[doc = "Reader of field `SDIO_INITI`"]
pub type SDIO_INITI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDIO_INITI`"]
pub struct SDIO_INITI_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_INITI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reader of field `SDIO_DCAP`"]
pub type SDIO_DCAP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDIO_DCAP`"]
pub struct SDIO_DCAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_DCAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Reader of field `SDIO_DTHDRV`"]
pub type SDIO_DTHDRV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDIO_DTHDRV`"]
pub struct SDIO_DTHDRV_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_DTHDRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "Reader of field `SDIO_TIMER_TARGET`"]
pub type SDIO_TIMER_TARGET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDIO_TIMER_TARGET`"]
pub struct SDIO_TIMER_TARGET_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_TIMER_TARGET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn xpd_sdio_reg(&self) -> XPD_SDIO_REG_R {
        XPD_SDIO_REG_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn drefh_sdio(&self) -> DREFH_SDIO_R {
        DREFH_SDIO_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn drefm_sdio(&self) -> DREFM_SDIO_R {
        DREFM_SDIO_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn drefl_sdio(&self) -> DREFL_SDIO_R {
        DREFL_SDIO_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn reg1p8_ready(&self) -> REG1P8_READY_R {
        REG1P8_READY_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sdio_tieh(&self) -> SDIO_TIEH_R {
        SDIO_TIEH_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn sdio_force(&self) -> SDIO_FORCE_R {
        SDIO_FORCE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn sdio_pd_en(&self) -> SDIO_PD_EN_R {
        SDIO_PD_EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn sdio_encurlim(&self) -> SDIO_ENCURLIM_R {
        SDIO_ENCURLIM_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn sdio_modecurlim(&self) -> SDIO_MODECURLIM_R {
        SDIO_MODECURLIM_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn sdio_dcurlim(&self) -> SDIO_DCURLIM_R {
        SDIO_DCURLIM_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn sdio_en_initi(&self) -> SDIO_EN_INITI_R {
        SDIO_EN_INITI_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn sdio_initi(&self) -> SDIO_INITI_R {
        SDIO_INITI_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn sdio_dcap(&self) -> SDIO_DCAP_R {
        SDIO_DCAP_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn sdio_dthdrv(&self) -> SDIO_DTHDRV_R {
        SDIO_DTHDRV_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sdio_timer_target(&self) -> SDIO_TIMER_TARGET_R {
        SDIO_TIMER_TARGET_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn xpd_sdio_reg(&mut self) -> XPD_SDIO_REG_W {
        XPD_SDIO_REG_W { w: self }
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn drefh_sdio(&mut self) -> DREFH_SDIO_W {
        DREFH_SDIO_W { w: self }
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn drefm_sdio(&mut self) -> DREFM_SDIO_W {
        DREFM_SDIO_W { w: self }
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn drefl_sdio(&mut self) -> DREFL_SDIO_W {
        DREFL_SDIO_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sdio_tieh(&mut self) -> SDIO_TIEH_W {
        SDIO_TIEH_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn sdio_force(&mut self) -> SDIO_FORCE_W {
        SDIO_FORCE_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn sdio_pd_en(&mut self) -> SDIO_PD_EN_W {
        SDIO_PD_EN_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn sdio_encurlim(&mut self) -> SDIO_ENCURLIM_W {
        SDIO_ENCURLIM_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn sdio_modecurlim(&mut self) -> SDIO_MODECURLIM_W {
        SDIO_MODECURLIM_W { w: self }
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn sdio_dcurlim(&mut self) -> SDIO_DCURLIM_W {
        SDIO_DCURLIM_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn sdio_en_initi(&mut self) -> SDIO_EN_INITI_W {
        SDIO_EN_INITI_W { w: self }
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn sdio_initi(&mut self) -> SDIO_INITI_W {
        SDIO_INITI_W { w: self }
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn sdio_dcap(&mut self) -> SDIO_DCAP_W {
        SDIO_DCAP_W { w: self }
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn sdio_dthdrv(&mut self) -> SDIO_DTHDRV_W {
        SDIO_DTHDRV_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sdio_timer_target(&mut self) -> SDIO_TIMER_TARGET_W {
        SDIO_TIMER_TARGET_W { w: self }
    }
}
