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
    #[doc = "3: Reset System, but not RTC"]
    RESETSYSTEM = 3,
    #[doc = "4: Reset System & RTC"]
    RESETRTC = 4,
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
    pub fn variant(&self) -> crate::Variant<u8, WDT_STG0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WDT_STG0_A::DISABLE),
            1 => Val(WDT_STG0_A::INTERRUPT),
            2 => Val(WDT_STG0_A::RESETCPU),
            3 => Val(WDT_STG0_A::RESETSYSTEM),
            4 => Val(WDT_STG0_A::RESETRTC),
            i => Res(i),
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
    #[doc = "Checks if the value of the field is `RESETRTC`"]
    #[inline(always)]
    pub fn is_reset_rtc(&self) -> bool {
        *self == WDT_STG0_A::RESETRTC
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
        unsafe { self.bits(variant.into()) }
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
    #[doc = "Reset System, but not RTC"]
    #[inline(always)]
    pub fn reset_system(self) -> &'a mut W {
        self.variant(WDT_STG0_A::RESETSYSTEM)
    }
    #[doc = "Reset System & RTC"]
    #[inline(always)]
    pub fn reset_rtc(self) -> &'a mut W {
        self.variant(WDT_STG0_A::RESETRTC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
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
        unsafe { self.bits(variant.into()) }
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
    #[doc = "Reset System, but not RTC"]
    #[inline(always)]
    pub fn reset_system(self) -> &'a mut W {
        self.variant(WDT_STG0_A::RESETSYSTEM)
    }
    #[doc = "Reset System & RTC"]
    #[inline(always)]
    pub fn reset_rtc(self) -> &'a mut W {
        self.variant(WDT_STG0_A::RESETRTC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | (((value as u32) & 0x07) << 25);
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
        unsafe { self.bits(variant.into()) }
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
    #[doc = "Reset System, but not RTC"]
    #[inline(always)]
    pub fn reset_system(self) -> &'a mut W {
        self.variant(WDT_STG0_A::RESETSYSTEM)
    }
    #[doc = "Reset System & RTC"]
    #[inline(always)]
    pub fn reset_rtc(self) -> &'a mut W {
        self.variant(WDT_STG0_A::RESETRTC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 22)) | (((value as u32) & 0x07) << 22);
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
        unsafe { self.bits(variant.into()) }
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
    #[doc = "Reset System, but not RTC"]
    #[inline(always)]
    pub fn reset_system(self) -> &'a mut W {
        self.variant(WDT_STG0_A::RESETSYSTEM)
    }
    #[doc = "Reset System & RTC"]
    #[inline(always)]
    pub fn reset_rtc(self) -> &'a mut W {
        self.variant(WDT_STG0_A::RESETRTC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | (((value as u32) & 0x07) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn wdt_en(&self) -> WDT_EN_R {
        WDT_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn wdt_stg0(&self) -> WDT_STG0_R {
        WDT_STG0_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 25:27"]
    #[inline(always)]
    pub fn wdt_stg1(&self) -> WDT_STG1_R {
        WDT_STG1_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    #[doc = "Bits 22:24"]
    #[inline(always)]
    pub fn wdt_stg2(&self) -> WDT_STG2_R {
        WDT_STG2_R::new(((self.bits >> 22) & 0x07) as u8)
    }
    #[doc = "Bits 19:21"]
    #[inline(always)]
    pub fn wdt_stg3(&self) -> WDT_STG3_R {
        WDT_STG3_R::new(((self.bits >> 19) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn wdt_en(&mut self) -> WDT_EN_W {
        WDT_EN_W { w: self }
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn wdt_stg0(&mut self) -> WDT_STG0_W {
        WDT_STG0_W { w: self }
    }
    #[doc = "Bits 25:27"]
    #[inline(always)]
    pub fn wdt_stg1(&mut self) -> WDT_STG1_W {
        WDT_STG1_W { w: self }
    }
    #[doc = "Bits 22:24"]
    #[inline(always)]
    pub fn wdt_stg2(&mut self) -> WDT_STG2_W {
        WDT_STG2_W { w: self }
    }
    #[doc = "Bits 19:21"]
    #[inline(always)]
    pub fn wdt_stg3(&mut self) -> WDT_STG3_W {
        WDT_STG3_W { w: self }
    }
}
