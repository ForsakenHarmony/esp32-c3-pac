#[doc = "Reader of register USER2"]
pub type R = crate::R<u32, super::USER2>;
#[doc = "Writer for register USER2"]
pub type W = crate::W<u32, super::USER2>;
#[doc = "Register USER2 `reset()`'s with value 0"]
impl crate::ResetValue for super::USER2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USR_COMMAND_BITLEN`"]
pub type USR_COMMAND_BITLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USR_COMMAND_BITLEN`"]
pub struct USR_COMMAND_BITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_COMMAND_BITLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `MST_REMPTY_ERR_END_EN`"]
pub type MST_REMPTY_ERR_END_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MST_REMPTY_ERR_END_EN`"]
pub struct MST_REMPTY_ERR_END_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MST_REMPTY_ERR_END_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `USR_COMMAND_VALUE`"]
pub type USR_COMMAND_VALUE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `USR_COMMAND_VALUE`"]
pub struct USR_COMMAND_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_COMMAND_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn usr_command_bitlen(&self) -> USR_COMMAND_BITLEN_R {
        USR_COMMAND_BITLEN_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn mst_rempty_err_end_en(&self) -> MST_REMPTY_ERR_END_EN_R {
        MST_REMPTY_ERR_END_EN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn usr_command_value(&self) -> USR_COMMAND_VALUE_R {
        USR_COMMAND_VALUE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn usr_command_bitlen(&mut self) -> USR_COMMAND_BITLEN_W {
        USR_COMMAND_BITLEN_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn mst_rempty_err_end_en(&mut self) -> MST_REMPTY_ERR_END_EN_W {
        MST_REMPTY_ERR_END_EN_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn usr_command_value(&mut self) -> USR_COMMAND_VALUE_W {
        USR_COMMAND_VALUE_W { w: self }
    }
}
