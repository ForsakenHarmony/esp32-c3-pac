#[doc = "Reader of register DIG_PAD_HOLD"]
pub type R = crate::R<u32, super::DIG_PAD_HOLD>;
#[doc = "Writer for register DIG_PAD_HOLD"]
pub type W = crate::W<u32, super::DIG_PAD_HOLD>;
#[doc = "Register DIG_PAD_HOLD `reset()`'s with value 0"]
impl crate::ResetValue for super::DIG_PAD_HOLD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIG_PAD_HOLD`"]
pub type DIG_PAD_HOLD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DIG_PAD_HOLD`"]
pub struct DIG_PAD_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_PAD_HOLD_W<'a> {
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
    pub fn dig_pad_hold(&self) -> DIG_PAD_HOLD_R {
        DIG_PAD_HOLD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dig_pad_hold(&mut self) -> DIG_PAD_HOLD_W {
        DIG_PAD_HOLD_W { w: self }
    }
}
