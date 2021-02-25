#[doc = "Reader of register WDTCONFIG0"]
pub type R = crate::R<u32, super::WDTCONFIG0>;
#[doc = "Writer for register WDTCONFIG0"]
pub type W = crate::W<u32, super::WDTCONFIG0>;
#[doc = "Register WDTCONFIG0 `reset()`'s with value 0"]
impl crate::ResetValue for super::WDTCONFIG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDT_EN`"]
pub type WDT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDT_EN`"]
pub struct WDT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_EN_W<'a> {
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WDT_STG0_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Trigger an interrupt"]
    INTERRUPT = 1,
    #[doc = "2: Reset CPU core"]
    RESETCPU = 2,
    #[doc = "3: Reset System"]
    RESETSYSTEM = 3,
}
impl From<WDT_STG0_A> for u8 {
    #[inline(always)]
    fn from(variant: WDT_STG0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WDT_STG0`"]
pub type WDT_STG0_R = crate::R<u8, WDT_STG0_A>;
impl WDT_STG0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDT_STG0_A {
        match self.bits {
            0 => WDT_STG0_A::DISABLE,
            1 => WDT_STG0_A::INTERRUPT,
            2 => WDT_STG0_A::RESETCPU,
            3 => WDT_STG0_A::RESETSYSTEM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WDT_STG0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == WDT_STG0_A::INTERRUPT
    }
    #[doc = "Checks if the value of the field is `RESETCPU`"]
    #[inline(always)]
    pub fn is_reset_cpu(&self) -> bool {
        *self == WDT_STG0_A::RESETCPU
    }
    #[doc = "Checks if the value of the field is `RESETSYSTEM`"]
    #[inline(always)]
    pub fn is_reset_system(&self) -> bool {
        *self == WDT_STG0_A::RESETSYSTEM
    }
}
#[doc = "Write proxy for field `WDT_STG0`"]
pub struct WDT_STG0_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_STG0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDT_STG0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WDT_STG0_A::DISABLE)
    }
    #[doc = "Trigger an interrupt"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(WDT_STG0_A::INTERRUPT)
    }
    #[doc = "Reset CPU core"]
    #[inline(always)]
    pub fn reset_cpu(self) -> &'a mut W {
        self.variant(WDT_STG0_A::RESETCPU)
    }
    #[doc = "Reset System"]
    #[inline(always)]
    pub fn reset_system(self) -> &'a mut W {
        self.variant(WDT_STG0_A::RESETSYSTEM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
#[doc = ""]
pub type WDT_STG1_A = WDT_STG0_A;
#[doc = "Reader of field `WDT_STG1`"]
pub type WDT_STG1_R = crate::R<u8, WDT_STG0_A>;
#[doc = "Write proxy for field `WDT_STG1`"]
pub struct WDT_STG1_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_STG1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDT_STG1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WDT_STG0_A::DISABLE)
    }
    #[doc = "Trigger an interrupt"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(WDT_STG0_A::INTERRUPT)
    }
    #[doc = "Reset CPU core"]
    #[inline(always)]
    pub fn reset_cpu(self) -> &'a mut W {
        self.variant(WDT_STG0_A::RESETCPU)
    }
    #[doc = "Reset System"]
    #[inline(always)]
    pub fn reset_system(self) -> &'a mut W {
        self.variant(WDT_STG0_A::RESETSYSTEM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
        self.w
    }
}
#[doc = ""]
pub type WDT_STG2_A = WDT_STG0_A;
#[doc = "Reader of field `WDT_STG2`"]
pub type WDT_STG2_R = crate::R<u8, WDT_STG0_A>;
#[doc = "Write proxy for field `WDT_STG2`"]
pub struct WDT_STG2_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_STG2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDT_STG2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WDT_STG0_A::DISABLE)
    }
    #[doc = "Trigger an interrupt"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(WDT_STG0_A::INTERRUPT)
    }
    #[doc = "Reset CPU core"]
    #[inline(always)]
    pub fn reset_cpu(self) -> &'a mut W {
        self.variant(WDT_STG0_A::RESETCPU)
    }
    #[doc = "Reset System"]
    #[inline(always)]
    pub fn reset_system(self) -> &'a mut W {
        self.variant(WDT_STG0_A::RESETSYSTEM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
#[doc = ""]
pub type WDT_STG3_A = WDT_STG0_A;
#[doc = "Reader of field `WDT_STG3`"]
pub type WDT_STG3_R = crate::R<u8, WDT_STG0_A>;
#[doc = "Write proxy for field `WDT_STG3`"]
pub struct WDT_STG3_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_STG3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDT_STG3_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WDT_STG0_A::DISABLE)
    }
    #[doc = "Trigger an interrupt"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(WDT_STG0_A::INTERRUPT)
    }
    #[doc = "Reset CPU core"]
    #[inline(always)]
    pub fn reset_cpu(self) -> &'a mut W {
        self.variant(WDT_STG0_A::RESETCPU)
    }
    #[doc = "Reset System"]
    #[inline(always)]
    pub fn reset_system(self) -> &'a mut W {
        self.variant(WDT_STG0_A::RESETSYSTEM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 23)) | (((value as u32) & 0x03) << 23);
        self.w
    }
}
#[doc = "Write proxy for field `WDT_CONF_UPDATE_EN`"]
pub struct WDT_CONF_UPDATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_CONF_UPDATE_EN_W<'a> {
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
#[doc = "Reader of field `WDT_USE_XTAL`"]
pub type WDT_USE_XTAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDT_USE_XTAL`"]
pub struct WDT_USE_XTAL_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_USE_XTAL_W<'a> {
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WDT_CPU_RESET_LENGTH_A {
    #[doc = "0: 100ns"]
    T100NS = 0,
    #[doc = "1: 200ns"]
    T200NS = 1,
    #[doc = "2: 300ns"]
    T300NS = 2,
    #[doc = "3: 400ns"]
    T400NS = 3,
    #[doc = "4: 500ns"]
    T500NS = 4,
    #[doc = "5: 800ns"]
    T800NS = 5,
    #[doc = "6: 1600ns"]
    T1600NS = 6,
    #[doc = "7: 3200ns"]
    T3200NS = 7,
}
impl From<WDT_CPU_RESET_LENGTH_A> for u8 {
    #[inline(always)]
    fn from(variant: WDT_CPU_RESET_LENGTH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WDT_CPU_RESET_LENGTH`"]
pub type WDT_CPU_RESET_LENGTH_R = crate::R<u8, WDT_CPU_RESET_LENGTH_A>;
impl WDT_CPU_RESET_LENGTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDT_CPU_RESET_LENGTH_A {
        match self.bits {
            0 => WDT_CPU_RESET_LENGTH_A::T100NS,
            1 => WDT_CPU_RESET_LENGTH_A::T200NS,
            2 => WDT_CPU_RESET_LENGTH_A::T300NS,
            3 => WDT_CPU_RESET_LENGTH_A::T400NS,
            4 => WDT_CPU_RESET_LENGTH_A::T500NS,
            5 => WDT_CPU_RESET_LENGTH_A::T800NS,
            6 => WDT_CPU_RESET_LENGTH_A::T1600NS,
            7 => WDT_CPU_RESET_LENGTH_A::T3200NS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `T100NS`"]
    #[inline(always)]
    pub fn is_t100ns(&self) -> bool {
        *self == WDT_CPU_RESET_LENGTH_A::T100NS
    }
    #[doc = "Checks if the value of the field is `T200NS`"]
    #[inline(always)]
    pub fn is_t200ns(&self) -> bool {
        *self == WDT_CPU_RESET_LENGTH_A::T200NS
    }
    #[doc = "Checks if the value of the field is `T300NS`"]
    #[inline(always)]
    pub fn is_t300ns(&self) -> bool {
        *self == WDT_CPU_RESET_LENGTH_A::T300NS
    }
    #[doc = "Checks if the value of the field is `T400NS`"]
    #[inline(always)]
    pub fn is_t400ns(&self) -> bool {
        *self == WDT_CPU_RESET_LENGTH_A::T400NS
    }
    #[doc = "Checks if the value of the field is `T500NS`"]
    #[inline(always)]
    pub fn is_t500ns(&self) -> bool {
        *self == WDT_CPU_RESET_LENGTH_A::T500NS
    }
    #[doc = "Checks if the value of the field is `T800NS`"]
    #[inline(always)]
    pub fn is_t800ns(&self) -> bool {
        *self == WDT_CPU_RESET_LENGTH_A::T800NS
    }
    #[doc = "Checks if the value of the field is `T1600NS`"]
    #[inline(always)]
    pub fn is_t1600ns(&self) -> bool {
        *self == WDT_CPU_RESET_LENGTH_A::T1600NS
    }
    #[doc = "Checks if the value of the field is `T3200NS`"]
    #[inline(always)]
    pub fn is_t3200ns(&self) -> bool {
        *self == WDT_CPU_RESET_LENGTH_A::T3200NS
    }
}
#[doc = "Write proxy for field `WDT_CPU_RESET_LENGTH`"]
pub struct WDT_CPU_RESET_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_CPU_RESET_LENGTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDT_CPU_RESET_LENGTH_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "100ns"]
    #[inline(always)]
    pub fn t100ns(self) -> &'a mut W {
        self.variant(WDT_CPU_RESET_LENGTH_A::T100NS)
    }
    #[doc = "200ns"]
    #[inline(always)]
    pub fn t200ns(self) -> &'a mut W {
        self.variant(WDT_CPU_RESET_LENGTH_A::T200NS)
    }
    #[doc = "300ns"]
    #[inline(always)]
    pub fn t300ns(self) -> &'a mut W {
        self.variant(WDT_CPU_RESET_LENGTH_A::T300NS)
    }
    #[doc = "400ns"]
    #[inline(always)]
    pub fn t400ns(self) -> &'a mut W {
        self.variant(WDT_CPU_RESET_LENGTH_A::T400NS)
    }
    #[doc = "500ns"]
    #[inline(always)]
    pub fn t500ns(self) -> &'a mut W {
        self.variant(WDT_CPU_RESET_LENGTH_A::T500NS)
    }
    #[doc = "800ns"]
    #[inline(always)]
    pub fn t800ns(self) -> &'a mut W {
        self.variant(WDT_CPU_RESET_LENGTH_A::T800NS)
    }
    #[doc = "1600ns"]
    #[inline(always)]
    pub fn t1600ns(self) -> &'a mut W {
        self.variant(WDT_CPU_RESET_LENGTH_A::T1600NS)
    }
    #[doc = "3200ns"]
    #[inline(always)]
    pub fn t3200ns(self) -> &'a mut W {
        self.variant(WDT_CPU_RESET_LENGTH_A::T3200NS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = ""]
pub type WDT_SYS_RESET_LENGTH_A = WDT_CPU_RESET_LENGTH_A;
#[doc = "Reader of field `WDT_SYS_RESET_LENGTH`"]
pub type WDT_SYS_RESET_LENGTH_R = crate::R<u8, WDT_CPU_RESET_LENGTH_A>;
#[doc = "Write proxy for field `WDT_SYS_RESET_LENGTH`"]
pub struct WDT_SYS_RESET_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_SYS_RESET_LENGTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDT_SYS_RESET_LENGTH_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "100ns"]
    #[inline(always)]
    pub fn t100ns(self) -> &'a mut W {
        self.variant(WDT_CPU_RESET_LENGTH_A::T100NS)
    }
    #[doc = "200ns"]
    #[inline(always)]
    pub fn t200ns(self) -> &'a mut W {
        self.variant(WDT_CPU_RESET_LENGTH_A::T200NS)
    }
    #[doc = "300ns"]
    #[inline(always)]
    pub fn t300ns(self) -> &'a mut W {
        self.variant(WDT_CPU_RESET_LENGTH_A::T300NS)
    }
    #[doc = "400ns"]
    #[inline(always)]
    pub fn t400ns(self) -> &'a mut W {
        self.variant(WDT_CPU_RESET_LENGTH_A::T400NS)
    }
    #[doc = "500ns"]
    #[inline(always)]
    pub fn t500ns(self) -> &'a mut W {
        self.variant(WDT_CPU_RESET_LENGTH_A::T500NS)
    }
    #[doc = "800ns"]
    #[inline(always)]
    pub fn t800ns(self) -> &'a mut W {
        self.variant(WDT_CPU_RESET_LENGTH_A::T800NS)
    }
    #[doc = "1600ns"]
    #[inline(always)]
    pub fn t1600ns(self) -> &'a mut W {
        self.variant(WDT_CPU_RESET_LENGTH_A::T1600NS)
    }
    #[doc = "3200ns"]
    #[inline(always)]
    pub fn t3200ns(self) -> &'a mut W {
        self.variant(WDT_CPU_RESET_LENGTH_A::T3200NS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 15)) | (((value as u32) & 0x07) << 15);
        self.w
    }
}
#[doc = "Reader of field `WDT_FLASHBOOT_MOD_EN`"]
pub type WDT_FLASHBOOT_MOD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDT_FLASHBOOT_MOD_EN`"]
pub struct WDT_FLASHBOOT_MOD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_FLASHBOOT_MOD_EN_W<'a> {
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
#[doc = "Reader of field `WDT_PROCPU_RESET_EN`"]
pub type WDT_PROCPU_RESET_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDT_PROCPU_RESET_EN`"]
pub struct WDT_PROCPU_RESET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_PROCPU_RESET_EN_W<'a> {
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
#[doc = "Reader of field `WDT_APPCPU_RESET_EN`"]
pub type WDT_APPCPU_RESET_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDT_APPCPU_RESET_EN`"]
pub struct WDT_APPCPU_RESET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_APPCPU_RESET_EN_W<'a> {
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
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn wdt_en(&self) -> WDT_EN_R {
        WDT_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn wdt_stg0(&self) -> WDT_STG0_R {
        WDT_STG0_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn wdt_stg1(&self) -> WDT_STG1_R {
        WDT_STG1_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn wdt_stg2(&self) -> WDT_STG2_R {
        WDT_STG2_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bits 23:24"]
    #[inline(always)]
    pub fn wdt_stg3(&self) -> WDT_STG3_R {
        WDT_STG3_R::new(((self.bits >> 23) & 0x03) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn wdt_use_xtal(&self) -> WDT_USE_XTAL_R {
        WDT_USE_XTAL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn wdt_cpu_reset_length(&self) -> WDT_CPU_RESET_LENGTH_R {
        WDT_CPU_RESET_LENGTH_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn wdt_sys_reset_length(&self) -> WDT_SYS_RESET_LENGTH_R {
        WDT_SYS_RESET_LENGTH_R::new(((self.bits >> 15) & 0x07) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn wdt_flashboot_mod_en(&self) -> WDT_FLASHBOOT_MOD_EN_R {
        WDT_FLASHBOOT_MOD_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn wdt_procpu_reset_en(&self) -> WDT_PROCPU_RESET_EN_R {
        WDT_PROCPU_RESET_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn wdt_appcpu_reset_en(&self) -> WDT_APPCPU_RESET_EN_R {
        WDT_APPCPU_RESET_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn wdt_en(&mut self) -> WDT_EN_W {
        WDT_EN_W { w: self }
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn wdt_stg0(&mut self) -> WDT_STG0_W {
        WDT_STG0_W { w: self }
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn wdt_stg1(&mut self) -> WDT_STG1_W {
        WDT_STG1_W { w: self }
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn wdt_stg2(&mut self) -> WDT_STG2_W {
        WDT_STG2_W { w: self }
    }
    #[doc = "Bits 23:24"]
    #[inline(always)]
    pub fn wdt_stg3(&mut self) -> WDT_STG3_W {
        WDT_STG3_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn wdt_conf_update_en(&mut self) -> WDT_CONF_UPDATE_EN_W {
        WDT_CONF_UPDATE_EN_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn wdt_use_xtal(&mut self) -> WDT_USE_XTAL_W {
        WDT_USE_XTAL_W { w: self }
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn wdt_cpu_reset_length(&mut self) -> WDT_CPU_RESET_LENGTH_W {
        WDT_CPU_RESET_LENGTH_W { w: self }
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn wdt_sys_reset_length(&mut self) -> WDT_SYS_RESET_LENGTH_W {
        WDT_SYS_RESET_LENGTH_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn wdt_flashboot_mod_en(&mut self) -> WDT_FLASHBOOT_MOD_EN_W {
        WDT_FLASHBOOT_MOD_EN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn wdt_procpu_reset_en(&mut self) -> WDT_PROCPU_RESET_EN_W {
        WDT_PROCPU_RESET_EN_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn wdt_appcpu_reset_en(&mut self) -> WDT_APPCPU_RESET_EN_W {
        WDT_APPCPU_RESET_EN_W { w: self }
    }
}
