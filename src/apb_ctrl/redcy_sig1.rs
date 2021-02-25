#[doc = "Reader of register REDCY_SIG1"]
pub type R = crate::R<u32, super::REDCY_SIG1>;
#[doc = "Writer for register REDCY_SIG1"]
pub type W = crate::W<u32, super::REDCY_SIG1>;
#[doc = "Register REDCY_SIG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::REDCY_SIG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REDCY_NANDOR`"]
pub type REDCY_NANDOR_R = crate::R<bool, bool>;
#[doc = "Reader of field `REDCY_SIG1`"]
pub type REDCY_SIG1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `REDCY_SIG1`"]
pub struct REDCY_SIG1_W<'a> {
    w: &'a mut W,
}
impl<'a> REDCY_SIG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff_ffff) | ((value as u32) & 0x7fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn redcy_nandor(&self) -> REDCY_NANDOR_R {
        REDCY_NANDOR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:30"]
    #[inline(always)]
    pub fn redcy_sig1(&self) -> REDCY_SIG1_R {
        REDCY_SIG1_R::new((self.bits & 0x7fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:30"]
    #[inline(always)]
    pub fn redcy_sig1(&mut self) -> REDCY_SIG1_W {
        REDCY_SIG1_W { w: self }
    }
}
