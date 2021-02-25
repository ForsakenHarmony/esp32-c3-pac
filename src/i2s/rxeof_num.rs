#[doc = "Reader of register RXEOF_NUM"]
pub type R = crate::R<u32, super::RXEOF_NUM>;
#[doc = "Writer for register RXEOF_NUM"]
pub type W = crate::W<u32, super::RXEOF_NUM>;
#[doc = "Register RXEOF_NUM `reset()`'s with value 0"]
impl crate::ResetValue for super::RXEOF_NUM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RX_EOF_NUM`"]
pub type RX_EOF_NUM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RX_EOF_NUM`"]
pub struct RX_EOF_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_EOF_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn rx_eof_num(&self) -> RX_EOF_NUM_R {
        RX_EOF_NUM_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn rx_eof_num(&mut self) -> RX_EOF_NUM_W {
        RX_EOF_NUM_W { w: self }
    }
}
