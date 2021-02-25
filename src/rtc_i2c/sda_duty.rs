#[doc = "Reader of register SDA_DUTY"]
pub type R = crate::R<u32, super::SDA_DUTY>;
#[doc = "Writer for register SDA_DUTY"]
pub type W = crate::W<u32, super::SDA_DUTY>;
#[doc = "Register SDA_DUTY `reset()`'s with value 0"]
impl crate::ResetValue for super::SDA_DUTY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SDA_DUTY_NUM`"]
pub type SDA_DUTY_NUM_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SDA_DUTY_NUM`"]
pub struct SDA_DUTY_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SDA_DUTY_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn sda_duty_num(&self) -> SDA_DUTY_NUM_R {
        SDA_DUTY_NUM_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn sda_duty_num(&mut self) -> SDA_DUTY_NUM_W {
        SDA_DUTY_NUM_W { w: self }
    }
}
