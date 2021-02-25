#[doc = "Reader of register ANA_CONF"]
pub type R = crate::R<u32, super::ANA_CONF>;
#[doc = "Writer for register ANA_CONF"]
pub type W = crate::W<u32, super::ANA_CONF>;
#[doc = "Register ANA_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::ANA_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PLL_I2C_PU`"]
pub type PLL_I2C_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLL_I2C_PU`"]
pub struct PLL_I2C_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_I2C_PU_W<'a> {
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
#[doc = "Reader of field `CKGEN_I2C_PU`"]
pub type CKGEN_I2C_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CKGEN_I2C_PU`"]
pub struct CKGEN_I2C_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> CKGEN_I2C_PU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `RFRX_PBUS_PU`"]
pub type RFRX_PBUS_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFRX_PBUS_PU`"]
pub struct RFRX_PBUS_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RFRX_PBUS_PU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `TXRF_I2C_PU`"]
pub type TXRF_I2C_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXRF_I2C_PU`"]
pub struct TXRF_I2C_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRF_I2C_PU_W<'a> {
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
#[doc = "Reader of field `PVTMON_PU`"]
pub type PVTMON_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PVTMON_PU`"]
pub struct PVTMON_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> PVTMON_PU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `BBPLL_CAL_SLP_START`"]
pub type BBPLL_CAL_SLP_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BBPLL_CAL_SLP_START`"]
pub struct BBPLL_CAL_SLP_START_W<'a> {
    w: &'a mut W,
}
impl<'a> BBPLL_CAL_SLP_START_W<'a> {
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
#[doc = "Reader of field `PLLA_FORCE_PU`"]
pub type PLLA_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLA_FORCE_PU`"]
pub struct PLLA_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLA_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `PLLA_FORCE_PD`"]
pub type PLLA_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLA_FORCE_PD`"]
pub struct PLLA_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLA_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `SAR_I2C_PU`"]
pub type SAR_I2C_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAR_I2C_PU`"]
pub struct SAR_I2C_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_I2C_PU_W<'a> {
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
#[doc = "Reader of field `GLITCH_RST_EN`"]
pub type GLITCH_RST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GLITCH_RST_EN`"]
pub struct GLITCH_RST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GLITCH_RST_EN_W<'a> {
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
#[doc = "Reader of field `I2C_RESET_POR_FORCE_PU`"]
pub type I2C_RESET_POR_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_RESET_POR_FORCE_PU`"]
pub struct I2C_RESET_POR_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_RESET_POR_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `I2C_RESET_POR_FORCE_PD`"]
pub type I2C_RESET_POR_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_RESET_POR_FORCE_PD`"]
pub struct I2C_RESET_POR_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_RESET_POR_FORCE_PD_W<'a> {
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
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pll_i2c_pu(&self) -> PLL_I2C_PU_R {
        PLL_I2C_PU_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn ckgen_i2c_pu(&self) -> CKGEN_I2C_PU_R {
        CKGEN_I2C_PU_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rfrx_pbus_pu(&self) -> RFRX_PBUS_PU_R {
        RFRX_PBUS_PU_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn txrf_i2c_pu(&self) -> TXRF_I2C_PU_R {
        TXRF_I2C_PU_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn pvtmon_pu(&self) -> PVTMON_PU_R {
        PVTMON_PU_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn bbpll_cal_slp_start(&self) -> BBPLL_CAL_SLP_START_R {
        BBPLL_CAL_SLP_START_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn plla_force_pu(&self) -> PLLA_FORCE_PU_R {
        PLLA_FORCE_PU_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn plla_force_pd(&self) -> PLLA_FORCE_PD_R {
        PLLA_FORCE_PD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn sar_i2c_pu(&self) -> SAR_I2C_PU_R {
        SAR_I2C_PU_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn glitch_rst_en(&self) -> GLITCH_RST_EN_R {
        GLITCH_RST_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn i2c_reset_por_force_pu(&self) -> I2C_RESET_POR_FORCE_PU_R {
        I2C_RESET_POR_FORCE_PU_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn i2c_reset_por_force_pd(&self) -> I2C_RESET_POR_FORCE_PD_R {
        I2C_RESET_POR_FORCE_PD_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pll_i2c_pu(&mut self) -> PLL_I2C_PU_W {
        PLL_I2C_PU_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn ckgen_i2c_pu(&mut self) -> CKGEN_I2C_PU_W {
        CKGEN_I2C_PU_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rfrx_pbus_pu(&mut self) -> RFRX_PBUS_PU_W {
        RFRX_PBUS_PU_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn txrf_i2c_pu(&mut self) -> TXRF_I2C_PU_W {
        TXRF_I2C_PU_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn pvtmon_pu(&mut self) -> PVTMON_PU_W {
        PVTMON_PU_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn bbpll_cal_slp_start(&mut self) -> BBPLL_CAL_SLP_START_W {
        BBPLL_CAL_SLP_START_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn plla_force_pu(&mut self) -> PLLA_FORCE_PU_W {
        PLLA_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn plla_force_pd(&mut self) -> PLLA_FORCE_PD_W {
        PLLA_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn sar_i2c_pu(&mut self) -> SAR_I2C_PU_W {
        SAR_I2C_PU_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn glitch_rst_en(&mut self) -> GLITCH_RST_EN_W {
        GLITCH_RST_EN_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn i2c_reset_por_force_pu(&mut self) -> I2C_RESET_POR_FORCE_PU_W {
        I2C_RESET_POR_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn i2c_reset_por_force_pd(&mut self) -> I2C_RESET_POR_FORCE_PD_W {
        I2C_RESET_POR_FORCE_PD_W { w: self }
    }
}
