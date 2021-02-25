#[doc = "Reader of register CONF_SIGLE_DATA"]
pub type R = crate::R<u32, super::CONF_SIGLE_DATA>;
#[doc = "Writer for register CONF_SIGLE_DATA"]
pub type W = crate::W<u32, super::CONF_SIGLE_DATA>;
#[doc = "Register CONF_SIGLE_DATA `reset()`'s with value 0"]
impl crate::ResetValue for super::CONF_SIGLE_DATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SINGLE_DATA`"]
pub type SINGLE_DATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SINGLE_DATA`"]
pub struct SINGLE_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGLE_DATA_W<'a> {
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
    pub fn single_data(&self) -> SINGLE_DATA_R {
        SINGLE_DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn single_data(&mut self) -> SINGLE_DATA_W {
        SINGLE_DATA_W { w: self }
    }
}
