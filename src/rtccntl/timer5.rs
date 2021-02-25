#[doc = "Reader of register TIMER5"]
pub type R = crate::R<u32, super::TIMER5>;
#[doc = "Writer for register TIMER5"]
pub type W = crate::W<u32, super::TIMER5>;
#[doc = "Register TIMER5 `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMER5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MIN_SLP_VAL`"]
pub type MIN_SLP_VAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MIN_SLP_VAL`"]
pub struct MIN_SLP_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> MIN_SLP_VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn min_slp_val(&self) -> MIN_SLP_VAL_R {
        MIN_SLP_VAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn min_slp_val(&mut self) -> MIN_SLP_VAL_W {
        MIN_SLP_VAL_W { w: self }
    }
}
