#[doc = "Reader of register RTCCALICFG"]
pub type R = crate::R<u32, super::RTCCALICFG>;
#[doc = "Writer for register RTCCALICFG"]
pub type W = crate::W<u32, super::RTCCALICFG>;
#[doc = "Register RTCCALICFG `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCCALICFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `START`"]
pub type START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `START`"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
#[doc = "Reader of field `MAX`"]
pub type MAX_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MAX`"]
pub struct MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 16)) | (((value as u32) & 0x7fff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RDY`"]
pub type RDY_R = crate::R<bool, bool>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLK_SEL_A {
    #[doc = "0: Select RTC slow clock"]
    RTC_MUX = 0,
    #[doc = "1: Internal 8 MHz RC oscillator, divided by 256"]
    CK8M_D256 = 1,
    #[doc = "2: Select XTAL_32K"]
    XTAL32K = 2,
}
impl From<CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLK_SEL`"]
pub type CLK_SEL_R = crate::R<u8, CLK_SEL_A>;
impl CLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLK_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLK_SEL_A::RTC_MUX),
            1 => Val(CLK_SEL_A::CK8M_D256),
            2 => Val(CLK_SEL_A::XTAL32K),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RTC_MUX`"]
    #[inline(always)]
    pub fn is_rtc_mux(&self) -> bool {
        *self == CLK_SEL_A::RTC_MUX
    }
    #[doc = "Checks if the value of the field is `CK8M_D256`"]
    #[inline(always)]
    pub fn is_ck8m_d256(&self) -> bool {
        *self == CLK_SEL_A::CK8M_D256
    }
    #[doc = "Checks if the value of the field is `XTAL32K`"]
    #[inline(always)]
    pub fn is_xtal32k(&self) -> bool {
        *self == CLK_SEL_A::XTAL32K
    }
}
#[doc = "Write proxy for field `CLK_SEL`"]
pub struct CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select RTC slow clock"]
    #[inline(always)]
    pub fn rtc_mux(self) -> &'a mut W {
        self.variant(CLK_SEL_A::RTC_MUX)
    }
    #[doc = "Internal 8 MHz RC oscillator, divided by 256"]
    #[inline(always)]
    pub fn ck8m_d256(self) -> &'a mut W {
        self.variant(CLK_SEL_A::CK8M_D256)
    }
    #[doc = "Select XTAL_32K"]
    #[inline(always)]
    pub fn xtal32k(self) -> &'a mut W {
        self.variant(CLK_SEL_A::XTAL32K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reader of field `START_CYCLING`"]
pub type START_CYCLING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `START_CYCLING`"]
pub struct START_CYCLING_W<'a> {
    w: &'a mut W,
}
impl<'a> START_CYCLING_W<'a> {
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
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 16:30"]
    #[inline(always)]
    pub fn max(&self) -> MAX_R {
        MAX_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn clk_sel(&self) -> CLK_SEL_R {
        CLK_SEL_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn start_cycling(&self) -> START_CYCLING_R {
        START_CYCLING_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bits 16:30"]
    #[inline(always)]
    pub fn max(&mut self) -> MAX_W {
        MAX_W { w: self }
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn clk_sel(&mut self) -> CLK_SEL_W {
        CLK_SEL_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn start_cycling(&mut self) -> START_CYCLING_W {
        START_CYCLING_W { w: self }
    }
}
