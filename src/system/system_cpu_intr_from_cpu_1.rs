#[doc = "Reader of register SYSTEM_CPU_INTR_FROM_CPU_1"]
pub type R = crate::R<u32, super::SYSTEM_CPU_INTR_FROM_CPU_1>;
#[doc = "Writer for register SYSTEM_CPU_INTR_FROM_CPU_1"]
pub type W = crate::W<u32, super::SYSTEM_CPU_INTR_FROM_CPU_1>;
#[doc = "Register SYSTEM_CPU_INTR_FROM_CPU_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSTEM_CPU_INTR_FROM_CPU_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSTEM_CPU_INTR_FROM_CPU_1`"]
pub type SYSTEM_CPU_INTR_FROM_CPU_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_CPU_INTR_FROM_CPU_1`"]
pub struct SYSTEM_CPU_INTR_FROM_CPU_1_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_CPU_INTR_FROM_CPU_1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn system_cpu_intr_from_cpu_1(&self) -> SYSTEM_CPU_INTR_FROM_CPU_1_R {
        SYSTEM_CPU_INTR_FROM_CPU_1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn system_cpu_intr_from_cpu_1(&mut self) -> SYSTEM_CPU_INTR_FROM_CPU_1_W {
        SYSTEM_CPU_INTR_FROM_CPU_1_W { w: self }
    }
}
