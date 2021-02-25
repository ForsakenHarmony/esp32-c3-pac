#[doc = "Reader of register TIMER2"]
pub type R = crate::R<u32, super::TIMER2>;
#[doc = "Writer for register TIMER2"]
pub type W = crate::W<u32, super::TIMER2>;
#[doc = "Register TIMER2 `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMER2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MIN_TIME_CK8M_OFF`"]
pub type MIN_TIME_CK8M_OFF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MIN_TIME_CK8M_OFF`"]
pub struct MIN_TIME_CK8M_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> MIN_TIME_CK8M_OFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn min_time_ck8m_off(&self) -> MIN_TIME_CK8M_OFF_R {
        MIN_TIME_CK8M_OFF_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn min_time_ck8m_off(&mut self) -> MIN_TIME_CK8M_OFF_W {
        MIN_TIME_CK8M_OFF_W { w: self }
    }
}
