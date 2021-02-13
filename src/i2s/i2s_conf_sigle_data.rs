#[doc = "Reader of register I2S_CONF_SIGLE_DATA"]
pub type R = crate::R<u32, super::I2S_CONF_SIGLE_DATA>;
#[doc = "Writer for register I2S_CONF_SIGLE_DATA"]
pub type W = crate::W<u32, super::I2S_CONF_SIGLE_DATA>;
#[doc = "Register I2S_CONF_SIGLE_DATA `reset()`'s with value 0"]
impl crate::ResetValue for super::I2S_CONF_SIGLE_DATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2S_SINGLE_DATA`"]
pub type I2S_SINGLE_DATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `I2S_SINGLE_DATA`"]
pub struct I2S_SINGLE_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_SINGLE_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn i2s_single_data(&self) -> I2S_SINGLE_DATA_R {
        I2S_SINGLE_DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn i2s_single_data(&mut self) -> I2S_SINGLE_DATA_W {
        I2S_SINGLE_DATA_W { w: self }
    }
}
