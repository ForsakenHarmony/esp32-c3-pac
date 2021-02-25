#[doc = "Reader of register OPTIONS0"]
pub type R = crate::R<u32, super::OPTIONS0>;
#[doc = "Writer for register OPTIONS0"]
pub type W = crate::W<u32, super::OPTIONS0>;
#[doc = "Register OPTIONS0 `reset()`'s with value 0"]
impl crate::ResetValue for super::OPTIONS0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SW_SYS_RST`"]
pub struct SW_SYS_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_SYS_RST_W<'a> {
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
#[doc = "Reader of field `DG_WRAP_FORCE_NORST`"]
pub type DG_WRAP_FORCE_NORST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DG_WRAP_FORCE_NORST`"]
pub struct DG_WRAP_FORCE_NORST_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_WRAP_FORCE_NORST_W<'a> {
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
#[doc = "Reader of field `DG_WRAP_FORCE_RST`"]
pub type DG_WRAP_FORCE_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DG_WRAP_FORCE_RST`"]
pub struct DG_WRAP_FORCE_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_WRAP_FORCE_RST_W<'a> {
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
#[doc = "Reader of field `ANALOG_FORCE_NOISO`"]
pub type ANALOG_FORCE_NOISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ANALOG_FORCE_NOISO`"]
pub struct ANALOG_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> ANALOG_FORCE_NOISO_W<'a> {
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
#[doc = "Reader of field `PLL_FORCE_NOISO`"]
pub type PLL_FORCE_NOISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLL_FORCE_NOISO`"]
pub struct PLL_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_FORCE_NOISO_W<'a> {
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
#[doc = "Reader of field `XTL_FORCE_NOISO`"]
pub type XTL_FORCE_NOISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XTL_FORCE_NOISO`"]
pub struct XTL_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> XTL_FORCE_NOISO_W<'a> {
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
#[doc = "Reader of field `ANALOG_FORCE_ISO`"]
pub type ANALOG_FORCE_ISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ANALOG_FORCE_ISO`"]
pub struct ANALOG_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> ANALOG_FORCE_ISO_W<'a> {
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
#[doc = "Reader of field `PLL_FORCE_ISO`"]
pub type PLL_FORCE_ISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLL_FORCE_ISO`"]
pub struct PLL_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_FORCE_ISO_W<'a> {
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
#[doc = "Reader of field `XTL_FORCE_ISO`"]
pub type XTL_FORCE_ISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XTL_FORCE_ISO`"]
pub struct XTL_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> XTL_FORCE_ISO_W<'a> {
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
#[doc = "Reader of field `XTL_EXT_CTR_SEL`"]
pub type XTL_EXT_CTR_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XTL_EXT_CTR_SEL`"]
pub struct XTL_EXT_CTR_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> XTL_EXT_CTR_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `XTL_EN_WAIT`"]
pub type XTL_EN_WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XTL_EN_WAIT`"]
pub struct XTL_EN_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> XTL_EN_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 14)) | (((value as u32) & 0x0f) << 14);
        self.w
    }
}
#[doc = "Reader of field `XTL_FORCE_PU`"]
pub type XTL_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XTL_FORCE_PU`"]
pub struct XTL_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> XTL_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `XTL_FORCE_PD`"]
pub type XTL_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XTL_FORCE_PD`"]
pub struct XTL_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> XTL_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `BBPLL_FORCE_PU`"]
pub type BBPLL_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BBPLL_FORCE_PU`"]
pub struct BBPLL_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> BBPLL_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `BBPLL_FORCE_PD`"]
pub type BBPLL_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BBPLL_FORCE_PD`"]
pub struct BBPLL_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> BBPLL_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `BBPLL_I2C_FORCE_PU`"]
pub type BBPLL_I2C_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BBPLL_I2C_FORCE_PU`"]
pub struct BBPLL_I2C_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> BBPLL_I2C_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `BBPLL_I2C_FORCE_PD`"]
pub type BBPLL_I2C_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BBPLL_I2C_FORCE_PD`"]
pub struct BBPLL_I2C_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> BBPLL_I2C_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `BB_I2C_FORCE_PU`"]
pub type BB_I2C_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BB_I2C_FORCE_PU`"]
pub struct BB_I2C_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> BB_I2C_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `BB_I2C_FORCE_PD`"]
pub type BB_I2C_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BB_I2C_FORCE_PD`"]
pub struct BB_I2C_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> BB_I2C_FORCE_PD_W<'a> {
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
#[doc = "Write proxy for field `SW_PROCPU_RST`"]
pub struct SW_PROCPU_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_PROCPU_RST_W<'a> {
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
#[doc = "Write proxy for field `SW_APPCPU_RST`"]
pub struct SW_APPCPU_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_APPCPU_RST_W<'a> {
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
#[doc = "Reader of field `SW_STALL_PROCPU_C0`"]
pub type SW_STALL_PROCPU_C0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SW_STALL_PROCPU_C0`"]
pub struct SW_STALL_PROCPU_C0_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_STALL_PROCPU_C0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `SW_STALL_APPCPU_C0`"]
pub type SW_STALL_APPCPU_C0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SW_STALL_APPCPU_C0`"]
pub struct SW_STALL_APPCPU_C0_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_STALL_APPCPU_C0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn dg_wrap_force_norst(&self) -> DG_WRAP_FORCE_NORST_R {
        DG_WRAP_FORCE_NORST_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dg_wrap_force_rst(&self) -> DG_WRAP_FORCE_RST_R {
        DG_WRAP_FORCE_RST_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn analog_force_noiso(&self) -> ANALOG_FORCE_NOISO_R {
        ANALOG_FORCE_NOISO_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn pll_force_noiso(&self) -> PLL_FORCE_NOISO_R {
        PLL_FORCE_NOISO_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn xtl_force_noiso(&self) -> XTL_FORCE_NOISO_R {
        XTL_FORCE_NOISO_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn analog_force_iso(&self) -> ANALOG_FORCE_ISO_R {
        ANALOG_FORCE_ISO_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pll_force_iso(&self) -> PLL_FORCE_ISO_R {
        PLL_FORCE_ISO_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn xtl_force_iso(&self) -> XTL_FORCE_ISO_R {
        XTL_FORCE_ISO_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn xtl_ext_ctr_sel(&self) -> XTL_EXT_CTR_SEL_R {
        XTL_EXT_CTR_SEL_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 14:17"]
    #[inline(always)]
    pub fn xtl_en_wait(&self) -> XTL_EN_WAIT_R {
        XTL_EN_WAIT_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn xtl_force_pu(&self) -> XTL_FORCE_PU_R {
        XTL_FORCE_PU_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn xtl_force_pd(&self) -> XTL_FORCE_PD_R {
        XTL_FORCE_PD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn bbpll_force_pu(&self) -> BBPLL_FORCE_PU_R {
        BBPLL_FORCE_PU_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn bbpll_force_pd(&self) -> BBPLL_FORCE_PD_R {
        BBPLL_FORCE_PD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bbpll_i2c_force_pu(&self) -> BBPLL_I2C_FORCE_PU_R {
        BBPLL_I2C_FORCE_PU_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn bbpll_i2c_force_pd(&self) -> BBPLL_I2C_FORCE_PD_R {
        BBPLL_I2C_FORCE_PD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn bb_i2c_force_pu(&self) -> BB_I2C_FORCE_PU_R {
        BB_I2C_FORCE_PU_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn bb_i2c_force_pd(&self) -> BB_I2C_FORCE_PD_R {
        BB_I2C_FORCE_PD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn sw_stall_procpu_c0(&self) -> SW_STALL_PROCPU_C0_R {
        SW_STALL_PROCPU_C0_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sw_stall_appcpu_c0(&self) -> SW_STALL_APPCPU_C0_R {
        SW_STALL_APPCPU_C0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sw_sys_rst(&mut self) -> SW_SYS_RST_W {
        SW_SYS_RST_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn dg_wrap_force_norst(&mut self) -> DG_WRAP_FORCE_NORST_W {
        DG_WRAP_FORCE_NORST_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dg_wrap_force_rst(&mut self) -> DG_WRAP_FORCE_RST_W {
        DG_WRAP_FORCE_RST_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn analog_force_noiso(&mut self) -> ANALOG_FORCE_NOISO_W {
        ANALOG_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn pll_force_noiso(&mut self) -> PLL_FORCE_NOISO_W {
        PLL_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn xtl_force_noiso(&mut self) -> XTL_FORCE_NOISO_W {
        XTL_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn analog_force_iso(&mut self) -> ANALOG_FORCE_ISO_W {
        ANALOG_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pll_force_iso(&mut self) -> PLL_FORCE_ISO_W {
        PLL_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn xtl_force_iso(&mut self) -> XTL_FORCE_ISO_W {
        XTL_FORCE_ISO_W { w: self }
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn xtl_ext_ctr_sel(&mut self) -> XTL_EXT_CTR_SEL_W {
        XTL_EXT_CTR_SEL_W { w: self }
    }
    #[doc = "Bits 14:17"]
    #[inline(always)]
    pub fn xtl_en_wait(&mut self) -> XTL_EN_WAIT_W {
        XTL_EN_WAIT_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn xtl_force_pu(&mut self) -> XTL_FORCE_PU_W {
        XTL_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn xtl_force_pd(&mut self) -> XTL_FORCE_PD_W {
        XTL_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn bbpll_force_pu(&mut self) -> BBPLL_FORCE_PU_W {
        BBPLL_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn bbpll_force_pd(&mut self) -> BBPLL_FORCE_PD_W {
        BBPLL_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bbpll_i2c_force_pu(&mut self) -> BBPLL_I2C_FORCE_PU_W {
        BBPLL_I2C_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn bbpll_i2c_force_pd(&mut self) -> BBPLL_I2C_FORCE_PD_W {
        BBPLL_I2C_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn bb_i2c_force_pu(&mut self) -> BB_I2C_FORCE_PU_W {
        BB_I2C_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn bb_i2c_force_pd(&mut self) -> BB_I2C_FORCE_PD_W {
        BB_I2C_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sw_procpu_rst(&mut self) -> SW_PROCPU_RST_W {
        SW_PROCPU_RST_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sw_appcpu_rst(&mut self) -> SW_APPCPU_RST_W {
        SW_APPCPU_RST_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn sw_stall_procpu_c0(&mut self) -> SW_STALL_PROCPU_C0_W {
        SW_STALL_PROCPU_C0_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sw_stall_appcpu_c0(&mut self) -> SW_STALL_APPCPU_C0_W {
        SW_STALL_APPCPU_C0_W { w: self }
    }
}
