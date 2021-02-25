#[doc = "Reader of register SCL_LOW_PERIOD"]
pub type R = crate::R<u32, super::SCL_LOW_PERIOD>;
#[doc = "Writer for register SCL_LOW_PERIOD"]
pub type W = crate::W<u32, super::SCL_LOW_PERIOD>;
#[doc = "Register SCL_LOW_PERIOD `reset()`'s with value 0"]
impl crate::ResetValue for super::SCL_LOW_PERIOD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PERIOD`"]
pub type PERIOD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PERIOD`"]
pub struct PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn period(&mut self) -> PERIOD_W {
        PERIOD_W { w: self }
    }
}
