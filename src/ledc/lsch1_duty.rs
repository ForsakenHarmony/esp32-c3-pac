#[doc = "Reader of register LSCH1_DUTY"]
pub type R = crate::R<u32, super::LSCH1_DUTY>;
#[doc = "Writer for register LSCH1_DUTY"]
pub type W = crate::W<u32, super::LSCH1_DUTY>;
#[doc = "Register LSCH1_DUTY `reset()`'s with value 0"]
impl crate::ResetValue for super::LSCH1_DUTY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DUTY_LSCH1`"]
pub type DUTY_LSCH1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DUTY_LSCH1`"]
pub struct DUTY_LSCH1_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_LSCH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0007_ffff) | ((value as u32) & 0x0007_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn duty_lsch1(&self) -> DUTY_LSCH1_R {
        DUTY_LSCH1_R::new((self.bits & 0x0007_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn duty_lsch1(&mut self) -> DUTY_LSCH1_W {
        DUTY_LSCH1_W { w: self }
    }
}
