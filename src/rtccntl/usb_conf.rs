#[doc = "Reader of register USB_CONF"]
pub type R = crate::R<u32, super::USB_CONF>;
#[doc = "Writer for register USB_CONF"]
pub type W = crate::W<u32, super::USB_CONF>;
#[doc = "Register USB_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::USB_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IO_MUX_RESET_DISABLE`"]
pub type IO_MUX_RESET_DISABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IO_MUX_RESET_DISABLE`"]
pub struct IO_MUX_RESET_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_MUX_RESET_DISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn io_mux_reset_disable(&self) -> IO_MUX_RESET_DISABLE_R {
        IO_MUX_RESET_DISABLE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn io_mux_reset_disable(&mut self) -> IO_MUX_RESET_DISABLE_W {
        IO_MUX_RESET_DISABLE_W { w: self }
    }
}
