#[doc = "Reader of register WR_TIM_CONF1"]
pub type R = crate::R<u32, super::WR_TIM_CONF1>;
#[doc = "Writer for register WR_TIM_CONF1"]
pub type W = crate::W<u32, super::WR_TIM_CONF1>;
#[doc = "Register WR_TIM_CONF1 `reset()`'s with value 0"]
impl crate::ResetValue for super::WR_TIM_CONF1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PWR_ON_NUM`"]
pub type PWR_ON_NUM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PWR_ON_NUM`"]
pub struct PWR_ON_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_ON_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 8)) | (((value as u32) & 0xffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:23"]
    #[inline(always)]
    pub fn pwr_on_num(&self) -> PWR_ON_NUM_R {
        PWR_ON_NUM_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:23"]
    #[inline(always)]
    pub fn pwr_on_num(&mut self) -> PWR_ON_NUM_W {
        PWR_ON_NUM_W { w: self }
    }
}
