#[doc = "Reader of register RD_TIM_CONF"]
pub type R = crate::R<u32, super::RD_TIM_CONF>;
#[doc = "Writer for register RD_TIM_CONF"]
pub type W = crate::W<u32, super::RD_TIM_CONF>;
#[doc = "Register RD_TIM_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::RD_TIM_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `READ_INIT_NUM`"]
pub type READ_INIT_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `READ_INIT_NUM`"]
pub struct READ_INIT_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_INIT_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn read_init_num(&self) -> READ_INIT_NUM_R {
        READ_INIT_NUM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn read_init_num(&mut self) -> READ_INIT_NUM_W {
        READ_INIT_NUM_W { w: self }
    }
}
