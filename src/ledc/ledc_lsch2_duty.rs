#[doc = "Reader of register LEDC_LSCH2_DUTY"]
pub type R = crate::R<u32, super::LEDC_LSCH2_DUTY>;
#[doc = "Writer for register LEDC_LSCH2_DUTY"]
pub type W = crate::W<u32, super::LEDC_LSCH2_DUTY>;
#[doc = "Register LEDC_LSCH2_DUTY `reset()`'s with value 0"]
impl crate::ResetValue for super::LEDC_LSCH2_DUTY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LEDC_DUTY_LSCH2`"]
pub type LEDC_DUTY_LSCH2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LEDC_DUTY_LSCH2`"]
pub struct LEDC_DUTY_LSCH2_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DUTY_LSCH2_W<'a> {
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
    pub fn ledc_duty_lsch2(&self) -> LEDC_DUTY_LSCH2_R {
        LEDC_DUTY_LSCH2_R::new((self.bits & 0x0007_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn ledc_duty_lsch2(&mut self) -> LEDC_DUTY_LSCH2_W {
        LEDC_DUTY_LSCH2_W { w: self }
    }
}
