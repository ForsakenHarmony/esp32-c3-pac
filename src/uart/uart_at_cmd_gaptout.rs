#[doc = "Reader of register UART_AT_CMD_GAPTOUT"]
pub type R = crate::R<u32, super::UART_AT_CMD_GAPTOUT>;
#[doc = "Writer for register UART_AT_CMD_GAPTOUT"]
pub type W = crate::W<u32, super::UART_AT_CMD_GAPTOUT>;
#[doc = "Register UART_AT_CMD_GAPTOUT `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_AT_CMD_GAPTOUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_RX_GAP_TOUT`"]
pub type UART_RX_GAP_TOUT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `UART_RX_GAP_TOUT`"]
pub struct UART_RX_GAP_TOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RX_GAP_TOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn uart_rx_gap_tout(&self) -> UART_RX_GAP_TOUT_R {
        UART_RX_GAP_TOUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn uart_rx_gap_tout(&mut self) -> UART_RX_GAP_TOUT_W {
        UART_RX_GAP_TOUT_W { w: self }
    }
}
