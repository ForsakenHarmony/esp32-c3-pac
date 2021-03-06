#[doc = "Reader of register INTERRUPT_CORE0_CPU_INT_ENABLE"]
pub type R = crate::R<u32, super::INTERRUPT_CORE0_CPU_INT_ENABLE>;
#[doc = "Writer for register INTERRUPT_CORE0_CPU_INT_ENABLE"]
pub type W = crate::W<u32, super::INTERRUPT_CORE0_CPU_INT_ENABLE>;
#[doc = "Register INTERRUPT_CORE0_CPU_INT_ENABLE `reset()`'s with value 0"]
impl crate::ResetValue for super::INTERRUPT_CORE0_CPU_INT_ENABLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTERRUPT_CORE0_CPU_INT_ENABLE`"]
pub type INTERRUPT_CORE0_CPU_INT_ENABLE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `INTERRUPT_CORE0_CPU_INT_ENABLE`"]
pub struct INTERRUPT_CORE0_CPU_INT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERRUPT_CORE0_CPU_INT_ENABLE_W<'a> {
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
    pub fn interrupt_core0_cpu_int_enable(&self) -> INTERRUPT_CORE0_CPU_INT_ENABLE_R {
        INTERRUPT_CORE0_CPU_INT_ENABLE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn interrupt_core0_cpu_int_enable(&mut self) -> INTERRUPT_CORE0_CPU_INT_ENABLE_W {
        INTERRUPT_CORE0_CPU_INT_ENABLE_W { w: self }
    }
}
