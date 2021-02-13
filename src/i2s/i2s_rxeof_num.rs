#[doc = "Reader of register I2S_RXEOF_NUM"]
pub type R = crate::R<u32, super::I2S_RXEOF_NUM>;
#[doc = "Writer for register I2S_RXEOF_NUM"]
pub type W = crate::W<u32, super::I2S_RXEOF_NUM>;
#[doc = "Register I2S_RXEOF_NUM `reset()`'s with value 0"]
impl crate::ResetValue for super::I2S_RXEOF_NUM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2S_RX_EOF_NUM`"]
pub type I2S_RX_EOF_NUM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `I2S_RX_EOF_NUM`"]
pub struct I2S_RX_EOF_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_RX_EOF_NUM_W<'a> {
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
    pub fn i2s_rx_eof_num(&self) -> I2S_RX_EOF_NUM_R {
        I2S_RX_EOF_NUM_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn i2s_rx_eof_num(&mut self) -> I2S_RX_EOF_NUM_W {
        I2S_RX_EOF_NUM_W { w: self }
    }
}
