#[doc = "Reader of register UART_CLKDIV"]
pub type R = crate::R<u32, super::UART_CLKDIV>;
#[doc = "Writer for register UART_CLKDIV"]
pub type W = crate::W<u32, super::UART_CLKDIV>;
#[doc = "Register UART_CLKDIV `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_CLKDIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_CLKDIV_FRAG`"]
pub type UART_CLKDIV_FRAG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART_CLKDIV_FRAG`"]
pub struct UART_CLKDIV_FRAG_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_CLKDIV_FRAG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `UART_CLKDIV`"]
pub type UART_CLKDIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `UART_CLKDIV`"]
pub struct UART_CLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_CLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn uart_clkdiv_frag(&self) -> UART_CLKDIV_FRAG_R {
        UART_CLKDIV_FRAG_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn uart_clkdiv(&self) -> UART_CLKDIV_R {
        UART_CLKDIV_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn uart_clkdiv_frag(&mut self) -> UART_CLKDIV_FRAG_W {
        UART_CLKDIV_FRAG_W { w: self }
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn uart_clkdiv(&mut self) -> UART_CLKDIV_W {
        UART_CLKDIV_W { w: self }
    }
}
