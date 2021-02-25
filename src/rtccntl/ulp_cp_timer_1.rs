#[doc = "Reader of register ULP_CP_TIMER_1"]
pub type R = crate::R<u32, super::ULP_CP_TIMER_1>;
#[doc = "Writer for register ULP_CP_TIMER_1"]
pub type W = crate::W<u32, super::ULP_CP_TIMER_1>;
#[doc = "Register ULP_CP_TIMER_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ULP_CP_TIMER_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ULP_CP_TIMER_SLP_CYCLE`"]
pub type ULP_CP_TIMER_SLP_CYCLE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ULP_CP_TIMER_SLP_CYCLE`"]
pub struct ULP_CP_TIMER_SLP_CYCLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ULP_CP_TIMER_SLP_CYCLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn ulp_cp_timer_slp_cycle(&self) -> ULP_CP_TIMER_SLP_CYCLE_R {
        ULP_CP_TIMER_SLP_CYCLE_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn ulp_cp_timer_slp_cycle(&mut self) -> ULP_CP_TIMER_SLP_CYCLE_W {
        ULP_CP_TIMER_SLP_CYCLE_W { w: self }
    }
}
