#[doc = "Reader of register INTERRUPT_CORE0_MAC_INTR_MAP"]
pub type R = crate::R<u32, super::INTERRUPT_CORE0_MAC_INTR_MAP>;
#[doc = "Writer for register INTERRUPT_CORE0_MAC_INTR_MAP"]
pub type W = crate::W<u32, super::INTERRUPT_CORE0_MAC_INTR_MAP>;
#[doc = "Register INTERRUPT_CORE0_MAC_INTR_MAP `reset()`'s with value 0"]
impl crate::ResetValue for super::INTERRUPT_CORE0_MAC_INTR_MAP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTERRUPT_CORE0_MAC_INTR_MAP`"]
pub type INTERRUPT_CORE0_MAC_INTR_MAP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INTERRUPT_CORE0_MAC_INTR_MAP`"]
pub struct INTERRUPT_CORE0_MAC_INTR_MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERRUPT_CORE0_MAC_INTR_MAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn interrupt_core0_mac_intr_map(&self) -> INTERRUPT_CORE0_MAC_INTR_MAP_R {
        INTERRUPT_CORE0_MAC_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn interrupt_core0_mac_intr_map(&mut self) -> INTERRUPT_CORE0_MAC_INTR_MAP_W {
        INTERRUPT_CORE0_MAC_INTR_MAP_W { w: self }
    }
}
