#[doc = "Reader of register RTC_CNTL_TIMER6"]
pub type R = crate::R<u32, super::RTC_CNTL_TIMER6>;
#[doc = "Writer for register RTC_CNTL_TIMER6"]
pub type W = crate::W<u32, super::RTC_CNTL_TIMER6>;
#[doc = "Register RTC_CNTL_TIMER6 `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_TIMER6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_DG_PERI_POWERUP_TIMER`"]
pub type RTC_CNTL_DG_PERI_POWERUP_TIMER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_DG_PERI_POWERUP_TIMER`"]
pub struct RTC_CNTL_DG_PERI_POWERUP_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DG_PERI_POWERUP_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | (((value as u32) & 0x7f) << 25);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_DG_PERI_WAIT_TIMER`"]
pub type RTC_CNTL_DG_PERI_WAIT_TIMER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RTC_CNTL_DG_PERI_WAIT_TIMER`"]
pub struct RTC_CNTL_DG_PERI_WAIT_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DG_PERI_WAIT_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | (((value as u32) & 0x01ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn rtc_cntl_dg_peri_powerup_timer(&self) -> RTC_CNTL_DG_PERI_POWERUP_TIMER_R {
        RTC_CNTL_DG_PERI_POWERUP_TIMER_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn rtc_cntl_dg_peri_wait_timer(&self) -> RTC_CNTL_DG_PERI_WAIT_TIMER_R {
        RTC_CNTL_DG_PERI_WAIT_TIMER_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn rtc_cntl_dg_peri_powerup_timer(&mut self) -> RTC_CNTL_DG_PERI_POWERUP_TIMER_W {
        RTC_CNTL_DG_PERI_POWERUP_TIMER_W { w: self }
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn rtc_cntl_dg_peri_wait_timer(&mut self) -> RTC_CNTL_DG_PERI_WAIT_TIMER_W {
        RTC_CNTL_DG_PERI_WAIT_TIMER_W { w: self }
    }
}
