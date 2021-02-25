#[doc = "Reader of register LSCH5_HPOINT"]
pub type R = crate::R<u32, super::LSCH5_HPOINT>;
#[doc = "Writer for register LSCH5_HPOINT"]
pub type W = crate::W<u32, super::LSCH5_HPOINT>;
#[doc = "Register LSCH5_HPOINT `reset()`'s with value 0"]
impl crate::ResetValue for super::LSCH5_HPOINT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HPOINT_LSCH5`"]
pub type HPOINT_LSCH5_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HPOINT_LSCH5`"]
pub struct HPOINT_LSCH5_W<'a> {
    w: &'a mut W,
}
impl<'a> HPOINT_LSCH5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn hpoint_lsch5(&self) -> HPOINT_LSCH5_R {
        HPOINT_LSCH5_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn hpoint_lsch5(&mut self) -> HPOINT_LSCH5_W {
        HPOINT_LSCH5_W { w: self }
    }
}
