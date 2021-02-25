#[doc = "Reader of register INT_ENA_TIMERS"]
pub type R = crate::R<u32, super::INT_ENA_TIMERS>;
#[doc = "Writer for register INT_ENA_TIMERS"]
pub type W = crate::W<u32, super::INT_ENA_TIMERS>;
#[doc = "Register INT_ENA_TIMERS `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_ENA_TIMERS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDT_INT_ENA`"]
pub type WDT_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDT_INT_ENA`"]
pub struct WDT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_INT_ENA_W<'a> {
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
#[doc = "Reader of field `T0_INT_ENA`"]
pub type T0_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T0_INT_ENA`"]
pub struct T0_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_INT_ENA_W<'a> {
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
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wdt_int_ena(&self) -> WDT_INT_ENA_R {
        WDT_INT_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn t0_int_ena(&self) -> T0_INT_ENA_R {
        T0_INT_ENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wdt_int_ena(&mut self) -> WDT_INT_ENA_W {
        WDT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn t0_int_ena(&mut self) -> T0_INT_ENA_W {
        T0_INT_ENA_W { w: self }
    }
}
