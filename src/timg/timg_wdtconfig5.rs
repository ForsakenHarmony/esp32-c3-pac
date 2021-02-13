#[doc = "Reader of register TIMG_WDTCONFIG5"]
pub type R = crate::R<u32, super::TIMG_WDTCONFIG5>;
#[doc = "Writer for register TIMG_WDTCONFIG5"]
pub type W = crate::W<u32, super::TIMG_WDTCONFIG5>;
#[doc = "Register TIMG_WDTCONFIG5 `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMG_WDTCONFIG5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMG_WDT_STG3_HOLD`"]
pub type TIMG_WDT_STG3_HOLD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TIMG_WDT_STG3_HOLD`"]
pub struct TIMG_WDT_STG3_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_WDT_STG3_HOLD_W<'a> {
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
    pub fn timg_wdt_stg3_hold(&self) -> TIMG_WDT_STG3_HOLD_R {
        TIMG_WDT_STG3_HOLD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn timg_wdt_stg3_hold(&mut self) -> TIMG_WDT_STG3_HOLD_W {
        TIMG_WDT_STG3_HOLD_W { w: self }
    }
}
