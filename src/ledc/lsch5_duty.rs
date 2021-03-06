#[doc = "Reader of register LSCH5_DUTY"]
pub type R = crate::R<u32, super::LSCH5_DUTY>;
#[doc = "Writer for register LSCH5_DUTY"]
pub type W = crate::W<u32, super::LSCH5_DUTY>;
#[doc = "Register LSCH5_DUTY `reset()`'s with value 0"]
impl crate::ResetValue for super::LSCH5_DUTY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DUTY_LSCH5`"]
pub type DUTY_LSCH5_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DUTY_LSCH5`"]
pub struct DUTY_LSCH5_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_LSCH5_W<'a> {
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
    pub fn duty_lsch5(&self) -> DUTY_LSCH5_R {
        DUTY_LSCH5_R::new((self.bits & 0x0007_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn duty_lsch5(&mut self) -> DUTY_LSCH5_W {
        DUTY_LSCH5_W { w: self }
    }
}
