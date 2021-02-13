#[doc = "Reader of register RTC_CNTL_WDTWPROTECT"]
pub type R = crate::R<u32, super::RTC_CNTL_WDTWPROTECT>;
#[doc = "Writer for register RTC_CNTL_WDTWPROTECT"]
pub type W = crate::W<u32, super::RTC_CNTL_WDTWPROTECT>;
#[doc = "Register RTC_CNTL_WDTWPROTECT `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_WDTWPROTECT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_WDT_WKEY`"]
pub type RTC_CNTL_WDT_WKEY_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RTC_CNTL_WDT_WKEY`"]
pub struct RTC_CNTL_WDT_WKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_WDT_WKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_wkey(&self) -> RTC_CNTL_WDT_WKEY_R {
        RTC_CNTL_WDT_WKEY_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rtc_cntl_wdt_wkey(&mut self) -> RTC_CNTL_WDT_WKEY_W {
        RTC_CNTL_WDT_WKEY_W { w: self }
    }
}
