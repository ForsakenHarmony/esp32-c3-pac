#[doc = "Reader of register SCL_START_HOLD"]
pub type R = crate::R<u32, super::SCL_START_HOLD>;
#[doc = "Writer for register SCL_START_HOLD"]
pub type W = crate::W<u32, super::SCL_START_HOLD>;
#[doc = "Register SCL_START_HOLD `reset()`'s with value 0"]
impl crate::ResetValue for super::SCL_START_HOLD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIME`"]
pub type TIME_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TIME`"]
pub struct TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> TIME_W<'a> {
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
    pub fn time(&self) -> TIME_R {
        TIME_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn time(&mut self) -> TIME_W {
        TIME_W { w: self }
    }
}
