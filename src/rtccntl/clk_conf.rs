#[doc = "Reader of register CLK_CONF"]
pub type R = crate::R<u32, super::CLK_CONF>;
#[doc = "Writer for register CLK_CONF"]
pub type W = crate::W<u32, super::CLK_CONF>;
#[doc = "Register CLK_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ANA_CLK_RTC_SEL_A {
    #[doc = "0: Select slow clock"]
    SLOW_CK = 0,
    #[doc = "1: Select XTAL_32K"]
    CK_XTAL_32K = 1,
    #[doc = "2: Internal 8 MHz RC oscillator, divided by 256"]
    CK8M_D256_OUT = 2,
}
impl From<ANA_CLK_RTC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ANA_CLK_RTC_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ANA_CLK_RTC_SEL`"]
pub type ANA_CLK_RTC_SEL_R = crate::R<u8, ANA_CLK_RTC_SEL_A>;
impl ANA_CLK_RTC_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ANA_CLK_RTC_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ANA_CLK_RTC_SEL_A::SLOW_CK),
            1 => Val(ANA_CLK_RTC_SEL_A::CK_XTAL_32K),
            2 => Val(ANA_CLK_RTC_SEL_A::CK8M_D256_OUT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SLOW_CK`"]
    #[inline(always)]
    pub fn is_slow_ck(&self) -> bool {
        *self == ANA_CLK_RTC_SEL_A::SLOW_CK
    }
    #[doc = "Checks if the value of the field is `CK_XTAL_32K`"]
    #[inline(always)]
    pub fn is_ck_xtal_32k(&self) -> bool {
        *self == ANA_CLK_RTC_SEL_A::CK_XTAL_32K
    }
    #[doc = "Checks if the value of the field is `CK8M_D256_OUT`"]
    #[inline(always)]
    pub fn is_ck8m_d256_out(&self) -> bool {
        *self == ANA_CLK_RTC_SEL_A::CK8M_D256_OUT
    }
}
#[doc = "Write proxy for field `ANA_CLK_RTC_SEL`"]
pub struct ANA_CLK_RTC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ANA_CLK_RTC_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ANA_CLK_RTC_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select slow clock"]
    #[inline(always)]
    pub fn slow_ck(self) -> &'a mut W {
        self.variant(ANA_CLK_RTC_SEL_A::SLOW_CK)
    }
    #[doc = "Select XTAL_32K"]
    #[inline(always)]
    pub fn ck_xtal_32k(self) -> &'a mut W {
        self.variant(ANA_CLK_RTC_SEL_A::CK_XTAL_32K)
    }
    #[doc = "Internal 8 MHz RC oscillator, divided by 256"]
    #[inline(always)]
    pub fn ck8m_d256_out(self) -> &'a mut W {
        self.variant(ANA_CLK_RTC_SEL_A::CK8M_D256_OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAST_CLK_RTC_SEL_A {
    #[doc = "0: Select XTAL"]
    XTAL = 0,
    #[doc = "1: Select CK8M"]
    CK8M = 1,
}
impl From<FAST_CLK_RTC_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: FAST_CLK_RTC_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FAST_CLK_RTC_SEL`"]
pub type FAST_CLK_RTC_SEL_R = crate::R<bool, FAST_CLK_RTC_SEL_A>;
impl FAST_CLK_RTC_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAST_CLK_RTC_SEL_A {
        match self.bits {
            false => FAST_CLK_RTC_SEL_A::XTAL,
            true => FAST_CLK_RTC_SEL_A::CK8M,
        }
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == FAST_CLK_RTC_SEL_A::XTAL
    }
    #[doc = "Checks if the value of the field is `CK8M`"]
    #[inline(always)]
    pub fn is_ck8m(&self) -> bool {
        *self == FAST_CLK_RTC_SEL_A::CK8M
    }
}
#[doc = "Write proxy for field `FAST_CLK_RTC_SEL`"]
pub struct FAST_CLK_RTC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FAST_CLK_RTC_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAST_CLK_RTC_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Select XTAL"]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut W {
        self.variant(FAST_CLK_RTC_SEL_A::XTAL)
    }
    #[doc = "Select CK8M"]
    #[inline(always)]
    pub fn ck8m(self) -> &'a mut W {
        self.variant(FAST_CLK_RTC_SEL_A::CK8M)
    }
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
#[doc = "Reader of field `XTAL_GLOBAL_FORCE_NOGATING`"]
pub type XTAL_GLOBAL_FORCE_NOGATING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XTAL_GLOBAL_FORCE_NOGATING`"]
pub struct XTAL_GLOBAL_FORCE_NOGATING_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_GLOBAL_FORCE_NOGATING_W<'a> {
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
#[doc = "Reader of field `XTAL_GLOBAL_FORCE_GATING`"]
pub type XTAL_GLOBAL_FORCE_GATING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XTAL_GLOBAL_FORCE_GATING`"]
pub struct XTAL_GLOBAL_FORCE_GATING_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_GLOBAL_FORCE_GATING_W<'a> {
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CK8M_FORCE_PU_A {
    #[doc = "0: Don't force power up"]
    CLEAR = 0,
    #[doc = "1: Force power up"]
    FORCE = 1,
}
impl From<CK8M_FORCE_PU_A> for bool {
    #[inline(always)]
    fn from(variant: CK8M_FORCE_PU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CK8M_FORCE_PU`"]
pub type CK8M_FORCE_PU_R = crate::R<bool, CK8M_FORCE_PU_A>;
impl CK8M_FORCE_PU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CK8M_FORCE_PU_A {
        match self.bits {
            false => CK8M_FORCE_PU_A::CLEAR,
            true => CK8M_FORCE_PU_A::FORCE,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CK8M_FORCE_PU_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `FORCE`"]
    #[inline(always)]
    pub fn is_force(&self) -> bool {
        *self == CK8M_FORCE_PU_A::FORCE
    }
}
#[doc = "Write proxy for field `CK8M_FORCE_PU`"]
pub struct CK8M_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> CK8M_FORCE_PU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CK8M_FORCE_PU_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Don't force power up"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CK8M_FORCE_PU_A::CLEAR)
    }
    #[doc = "Force power up"]
    #[inline(always)]
    pub fn force(self) -> &'a mut W {
        self.variant(CK8M_FORCE_PU_A::FORCE)
    }
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CK8M_FORCE_PD_A {
    #[doc = "0: Don't force power down"]
    CLEAR = 0,
    #[doc = "1: Force power down"]
    FORCE = 1,
}
impl From<CK8M_FORCE_PD_A> for bool {
    #[inline(always)]
    fn from(variant: CK8M_FORCE_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CK8M_FORCE_PD`"]
pub type CK8M_FORCE_PD_R = crate::R<bool, CK8M_FORCE_PD_A>;
impl CK8M_FORCE_PD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CK8M_FORCE_PD_A {
        match self.bits {
            false => CK8M_FORCE_PD_A::CLEAR,
            true => CK8M_FORCE_PD_A::FORCE,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CK8M_FORCE_PD_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `FORCE`"]
    #[inline(always)]
    pub fn is_force(&self) -> bool {
        *self == CK8M_FORCE_PD_A::FORCE
    }
}
#[doc = "Write proxy for field `CK8M_FORCE_PD`"]
pub struct CK8M_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> CK8M_FORCE_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CK8M_FORCE_PD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Don't force power down"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CK8M_FORCE_PD_A::CLEAR)
    }
    #[doc = "Force power down"]
    #[inline(always)]
    pub fn force(self) -> &'a mut W {
        self.variant(CK8M_FORCE_PD_A::FORCE)
    }
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
#[doc = "Reader of field `CK8M_DFREQ`"]
pub type CK8M_DFREQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CK8M_DFREQ`"]
pub struct CK8M_DFREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CK8M_DFREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 17)) | (((value as u32) & 0xff) << 17);
        self.w
    }
}
#[doc = "Reader of field `CK8M_FORCE_NOGATING`"]
pub type CK8M_FORCE_NOGATING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CK8M_FORCE_NOGATING`"]
pub struct CK8M_FORCE_NOGATING_W<'a> {
    w: &'a mut W,
}
impl<'a> CK8M_FORCE_NOGATING_W<'a> {
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
#[doc = "Reader of field `XTAL_FORCE_NOGATING`"]
pub type XTAL_FORCE_NOGATING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XTAL_FORCE_NOGATING`"]
pub struct XTAL_FORCE_NOGATING_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_FORCE_NOGATING_W<'a> {
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
#[doc = "Reader of field `CK8M_DIV_SEL`"]
pub type CK8M_DIV_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CK8M_DIV_SEL`"]
pub struct CK8M_DIV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CK8M_DIV_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIG_CLK8M_EN_A {
    #[doc = "0: Disable CK8M"]
    DISABLE = 0,
    #[doc = "1: Enable CK8M for digital core (no relation to RTC core)"]
    ENABLE = 1,
}
impl From<DIG_CLK8M_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DIG_CLK8M_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIG_CLK8M_EN`"]
pub type DIG_CLK8M_EN_R = crate::R<bool, DIG_CLK8M_EN_A>;
impl DIG_CLK8M_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIG_CLK8M_EN_A {
        match self.bits {
            false => DIG_CLK8M_EN_A::DISABLE,
            true => DIG_CLK8M_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DIG_CLK8M_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DIG_CLK8M_EN_A::ENABLE
    }
}
#[doc = "Write proxy for field `DIG_CLK8M_EN`"]
pub struct DIG_CLK8M_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_CLK8M_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIG_CLK8M_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CK8M"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DIG_CLK8M_EN_A::DISABLE)
    }
    #[doc = "Enable CK8M for digital core (no relation to RTC core)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DIG_CLK8M_EN_A::ENABLE)
    }
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIG_CLK8M_D256_EN_A {
    #[doc = "0: Disable CK8M_D256_OUT"]
    DISABLE = 0,
    #[doc = "1: Enable CK8M_D256_OUT for digital core (no relation to RTC core)"]
    ENABLE = 1,
}
impl From<DIG_CLK8M_D256_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DIG_CLK8M_D256_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIG_CLK8M_D256_EN`"]
pub type DIG_CLK8M_D256_EN_R = crate::R<bool, DIG_CLK8M_D256_EN_A>;
impl DIG_CLK8M_D256_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIG_CLK8M_D256_EN_A {
        match self.bits {
            false => DIG_CLK8M_D256_EN_A::DISABLE,
            true => DIG_CLK8M_D256_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DIG_CLK8M_D256_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DIG_CLK8M_D256_EN_A::ENABLE
    }
}
#[doc = "Write proxy for field `DIG_CLK8M_D256_EN`"]
pub struct DIG_CLK8M_D256_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_CLK8M_D256_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIG_CLK8M_D256_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CK8M_D256_OUT"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DIG_CLK8M_D256_EN_A::DISABLE)
    }
    #[doc = "Enable CK8M_D256_OUT for digital core (no relation to RTC core)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DIG_CLK8M_D256_EN_A::ENABLE)
    }
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIG_XTAL32K_EN_A {
    #[doc = "0: Disable CK_XTAL_32K"]
    DISABLE = 0,
    #[doc = "1: Enable CK_XTAL_32K for digital core(no relation to RTC core)"]
    ENABLE = 1,
}
impl From<DIG_XTAL32K_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DIG_XTAL32K_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIG_XTAL32K_EN`"]
pub type DIG_XTAL32K_EN_R = crate::R<bool, DIG_XTAL32K_EN_A>;
impl DIG_XTAL32K_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIG_XTAL32K_EN_A {
        match self.bits {
            false => DIG_XTAL32K_EN_A::DISABLE,
            true => DIG_XTAL32K_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DIG_XTAL32K_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DIG_XTAL32K_EN_A::ENABLE
    }
}
#[doc = "Write proxy for field `DIG_XTAL32K_EN`"]
pub struct DIG_XTAL32K_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_XTAL32K_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIG_XTAL32K_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CK_XTAL_32K"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DIG_XTAL32K_EN_A::DISABLE)
    }
    #[doc = "Enable CK_XTAL_32K for digital core(no relation to RTC core)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DIG_XTAL32K_EN_A::ENABLE)
    }
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
#[doc = "Reader of field `ENB_CK8M_DIV`"]
pub type ENB_CK8M_DIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENB_CK8M_DIV`"]
pub struct ENB_CK8M_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ENB_CK8M_DIV_W<'a> {
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
#[doc = "Reader of field `ENB_CK8M`"]
pub type ENB_CK8M_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENB_CK8M`"]
pub struct ENB_CK8M_W<'a> {
    w: &'a mut W,
}
impl<'a> ENB_CK8M_W<'a> {
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CK8M_DIV_A {
    #[doc = "0: div128"]
    DIV128 = 0,
    #[doc = "1: div256"]
    DIV256 = 1,
    #[doc = "2: div512"]
    DIV512 = 2,
    #[doc = "3: div1024"]
    DIV1024 = 3,
}
impl From<CK8M_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CK8M_DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CK8M_DIV`"]
pub type CK8M_DIV_R = crate::R<u8, CK8M_DIV_A>;
impl CK8M_DIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CK8M_DIV_A {
        match self.bits {
            0 => CK8M_DIV_A::DIV128,
            1 => CK8M_DIV_A::DIV256,
            2 => CK8M_DIV_A::DIV512,
            3 => CK8M_DIV_A::DIV1024,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == CK8M_DIV_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == CK8M_DIV_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == CK8M_DIV_A::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == CK8M_DIV_A::DIV1024
    }
}
#[doc = "Write proxy for field `CK8M_DIV`"]
pub struct CK8M_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CK8M_DIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CK8M_DIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "div128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(CK8M_DIV_A::DIV128)
    }
    #[doc = "div256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(CK8M_DIV_A::DIV256)
    }
    #[doc = "div512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(CK8M_DIV_A::DIV512)
    }
    #[doc = "div1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut W {
        self.variant(CK8M_DIV_A::DIV1024)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `CK8M_DIV_SEL_VLD`"]
pub type CK8M_DIV_SEL_VLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CK8M_DIV_SEL_VLD`"]
pub struct CK8M_DIV_SEL_VLD_W<'a> {
    w: &'a mut W,
}
impl<'a> CK8M_DIV_SEL_VLD_W<'a> {
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
#[doc = "Reader of field `EFUSE_CLK_FORCE_NOGATING`"]
pub type EFUSE_CLK_FORCE_NOGATING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_CLK_FORCE_NOGATING`"]
pub struct EFUSE_CLK_FORCE_NOGATING_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_CLK_FORCE_NOGATING_W<'a> {
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
#[doc = "Reader of field `EFUSE_CLK_FORCE_GATING`"]
pub type EFUSE_CLK_FORCE_GATING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_CLK_FORCE_GATING`"]
pub struct EFUSE_CLK_FORCE_GATING_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_CLK_FORCE_GATING_W<'a> {
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
impl R {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn ana_clk_rtc_sel(&self) -> ANA_CLK_RTC_SEL_R {
        ANA_CLK_RTC_SEL_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn fast_clk_rtc_sel(&self) -> FAST_CLK_RTC_SEL_R {
        FAST_CLK_RTC_SEL_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn xtal_global_force_nogating(&self) -> XTAL_GLOBAL_FORCE_NOGATING_R {
        XTAL_GLOBAL_FORCE_NOGATING_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn xtal_global_force_gating(&self) -> XTAL_GLOBAL_FORCE_GATING_R {
        XTAL_GLOBAL_FORCE_GATING_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ck8m_force_pu(&self) -> CK8M_FORCE_PU_R {
        CK8M_FORCE_PU_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ck8m_force_pd(&self) -> CK8M_FORCE_PD_R {
        CK8M_FORCE_PD_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 17:24"]
    #[inline(always)]
    pub fn ck8m_dfreq(&self) -> CK8M_DFREQ_R {
        CK8M_DFREQ_R::new(((self.bits >> 17) & 0xff) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ck8m_force_nogating(&self) -> CK8M_FORCE_NOGATING_R {
        CK8M_FORCE_NOGATING_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn xtal_force_nogating(&self) -> XTAL_FORCE_NOGATING_R {
        XTAL_FORCE_NOGATING_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn ck8m_div_sel(&self) -> CK8M_DIV_SEL_R {
        CK8M_DIV_SEL_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dig_clk8m_en(&self) -> DIG_CLK8M_EN_R {
        DIG_CLK8M_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dig_clk8m_d256_en(&self) -> DIG_CLK8M_D256_EN_R {
        DIG_CLK8M_D256_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dig_xtal32k_en(&self) -> DIG_XTAL32K_EN_R {
        DIG_XTAL32K_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn enb_ck8m_div(&self) -> ENB_CK8M_DIV_R {
        ENB_CK8M_DIV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn enb_ck8m(&self) -> ENB_CK8M_R {
        ENB_CK8M_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn ck8m_div(&self) -> CK8M_DIV_R {
        CK8M_DIV_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ck8m_div_sel_vld(&self) -> CK8M_DIV_SEL_VLD_R {
        CK8M_DIV_SEL_VLD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn efuse_clk_force_nogating(&self) -> EFUSE_CLK_FORCE_NOGATING_R {
        EFUSE_CLK_FORCE_NOGATING_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn efuse_clk_force_gating(&self) -> EFUSE_CLK_FORCE_GATING_R {
        EFUSE_CLK_FORCE_GATING_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn ana_clk_rtc_sel(&mut self) -> ANA_CLK_RTC_SEL_W {
        ANA_CLK_RTC_SEL_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn fast_clk_rtc_sel(&mut self) -> FAST_CLK_RTC_SEL_W {
        FAST_CLK_RTC_SEL_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn xtal_global_force_nogating(&mut self) -> XTAL_GLOBAL_FORCE_NOGATING_W {
        XTAL_GLOBAL_FORCE_NOGATING_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn xtal_global_force_gating(&mut self) -> XTAL_GLOBAL_FORCE_GATING_W {
        XTAL_GLOBAL_FORCE_GATING_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ck8m_force_pu(&mut self) -> CK8M_FORCE_PU_W {
        CK8M_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ck8m_force_pd(&mut self) -> CK8M_FORCE_PD_W {
        CK8M_FORCE_PD_W { w: self }
    }
    #[doc = "Bits 17:24"]
    #[inline(always)]
    pub fn ck8m_dfreq(&mut self) -> CK8M_DFREQ_W {
        CK8M_DFREQ_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ck8m_force_nogating(&mut self) -> CK8M_FORCE_NOGATING_W {
        CK8M_FORCE_NOGATING_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn xtal_force_nogating(&mut self) -> XTAL_FORCE_NOGATING_W {
        XTAL_FORCE_NOGATING_W { w: self }
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn ck8m_div_sel(&mut self) -> CK8M_DIV_SEL_W {
        CK8M_DIV_SEL_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dig_clk8m_en(&mut self) -> DIG_CLK8M_EN_W {
        DIG_CLK8M_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dig_clk8m_d256_en(&mut self) -> DIG_CLK8M_D256_EN_W {
        DIG_CLK8M_D256_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dig_xtal32k_en(&mut self) -> DIG_XTAL32K_EN_W {
        DIG_XTAL32K_EN_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn enb_ck8m_div(&mut self) -> ENB_CK8M_DIV_W {
        ENB_CK8M_DIV_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn enb_ck8m(&mut self) -> ENB_CK8M_W {
        ENB_CK8M_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn ck8m_div(&mut self) -> CK8M_DIV_W {
        CK8M_DIV_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ck8m_div_sel_vld(&mut self) -> CK8M_DIV_SEL_VLD_W {
        CK8M_DIV_SEL_VLD_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn efuse_clk_force_nogating(&mut self) -> EFUSE_CLK_FORCE_NOGATING_W {
        EFUSE_CLK_FORCE_NOGATING_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn efuse_clk_force_gating(&mut self) -> EFUSE_CLK_FORCE_GATING_W {
        EFUSE_CLK_FORCE_GATING_W { w: self }
    }
}
