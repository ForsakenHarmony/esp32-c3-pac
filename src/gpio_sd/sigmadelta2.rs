#[doc = "Reader of register SIGMADELTA2"]
pub type R = crate::R<u32, super::SIGMADELTA2>;
#[doc = "Writer for register SIGMADELTA2"]
pub type W = crate::W<u32, super::SIGMADELTA2>;
#[doc = "Register SIGMADELTA2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SIGMADELTA2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SD2_PRESCALE`"]
pub type SD2_PRESCALE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SD2_PRESCALE`"]
pub struct SD2_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> SD2_PRESCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `SD2_IN`"]
pub type SD2_IN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SD2_IN`"]
pub struct SD2_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> SD2_IN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sd2_prescale(&self) -> SD2_PRESCALE_R {
        SD2_PRESCALE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sd2_in(&self) -> SD2_IN_R {
        SD2_IN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sd2_prescale(&mut self) -> SD2_PRESCALE_W {
        SD2_PRESCALE_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sd2_in(&mut self) -> SD2_IN_W {
        SD2_IN_W { w: self }
    }
}
