#[doc = "Reader of register CH2_RX_LIM"]
pub type R = crate::R<u32, super::CH2_RX_LIM>;
#[doc = "Writer for register CH2_RX_LIM"]
pub type W = crate::W<u32, super::CH2_RX_LIM>;
#[doc = "Register CH2_RX_LIM `reset()`'s with value 0"]
impl crate::ResetValue for super::CH2_RX_LIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RX_LIM_CH2`"]
pub type RX_LIM_CH2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RX_LIM_CH2`"]
pub struct RX_LIM_CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_LIM_CH2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn rx_lim_ch2(&self) -> RX_LIM_CH2_R {
        RX_LIM_CH2_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn rx_lim_ch2(&mut self) -> RX_LIM_CH2_W {
        RX_LIM_CH2_W { w: self }
    }
}
