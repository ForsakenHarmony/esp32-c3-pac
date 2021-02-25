#[doc = "Reader of register SCL_ST_TIME_OUT"]
pub type R = crate::R<u32, super::SCL_ST_TIME_OUT>;
#[doc = "Writer for register SCL_ST_TIME_OUT"]
pub type W = crate::W<u32, super::SCL_ST_TIME_OUT>;
#[doc = "Register SCL_ST_TIME_OUT `reset()`'s with value 0"]
impl crate::ResetValue for super::SCL_ST_TIME_OUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCL_ST_TO_REG`"]
pub type SCL_ST_TO_REG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCL_ST_TO_REG`"]
pub struct SCL_ST_TO_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_ST_TO_REG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn scl_st_to_reg(&self) -> SCL_ST_TO_REG_R {
        SCL_ST_TO_REG_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn scl_st_to_reg(&mut self) -> SCL_ST_TO_REG_W {
        SCL_ST_TO_REG_W { w: self }
    }
}
