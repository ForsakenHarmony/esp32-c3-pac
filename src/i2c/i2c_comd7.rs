#[doc = "Reader of register I2C_COMD7"]
pub type R = crate::R<u32, super::I2C_COMD7>;
#[doc = "Writer for register I2C_COMD7"]
pub type W = crate::W<u32, super::I2C_COMD7>;
#[doc = "Register I2C_COMD7 `reset()`'s with value 0"]
impl crate::ResetValue for super::I2C_COMD7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C_COMMAND7_DONE`"]
pub type I2C_COMMAND7_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_COMMAND7_DONE`"]
pub struct I2C_COMMAND7_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_COMMAND7_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `I2C_COMMAND7`"]
pub type I2C_COMMAND7_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `I2C_COMMAND7`"]
pub struct I2C_COMMAND7_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_COMMAND7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn i2c_command7_done(&self) -> I2C_COMMAND7_DONE_R {
        I2C_COMMAND7_DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn i2c_command7(&self) -> I2C_COMMAND7_R {
        I2C_COMMAND7_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn i2c_command7_done(&mut self) -> I2C_COMMAND7_DONE_W {
        I2C_COMMAND7_DONE_W { w: self }
    }
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn i2c_command7(&mut self) -> I2C_COMMAND7_W {
        I2C_COMMAND7_W { w: self }
    }
}
