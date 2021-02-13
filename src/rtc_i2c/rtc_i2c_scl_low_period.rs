#[doc = "Reader of register RTC_I2C_SCL_LOW_PERIOD"]
pub type R = crate::R<u32, super::RTC_I2C_SCL_LOW_PERIOD>;
#[doc = "Writer for register RTC_I2C_SCL_LOW_PERIOD"]
pub type W = crate::W<u32, super::RTC_I2C_SCL_LOW_PERIOD>;
#[doc = "Register RTC_I2C_SCL_LOW_PERIOD `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_I2C_SCL_LOW_PERIOD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_I2C_SCL_LOW_PERIOD`"]
pub type RTC_I2C_SCL_LOW_PERIOD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RTC_I2C_SCL_LOW_PERIOD`"]
pub struct RTC_I2C_SCL_LOW_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_SCL_LOW_PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn rtc_i2c_scl_low_period(&self) -> RTC_I2C_SCL_LOW_PERIOD_R {
        RTC_I2C_SCL_LOW_PERIOD_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn rtc_i2c_scl_low_period(&mut self) -> RTC_I2C_SCL_LOW_PERIOD_W {
        RTC_I2C_SCL_LOW_PERIOD_W { w: self }
    }
}
