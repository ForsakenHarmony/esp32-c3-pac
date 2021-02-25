#[doc = "Reader of register AT_CMD_PRECNT"]
pub type R = crate::R<u32, super::AT_CMD_PRECNT>;
#[doc = "Writer for register AT_CMD_PRECNT"]
pub type W = crate::W<u32, super::AT_CMD_PRECNT>;
#[doc = "Register AT_CMD_PRECNT `reset()`'s with value 0"]
impl crate::ResetValue for super::AT_CMD_PRECNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRE_IDLE_NUM`"]
pub type PRE_IDLE_NUM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PRE_IDLE_NUM`"]
pub struct PRE_IDLE_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> PRE_IDLE_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pre_idle_num(&self) -> PRE_IDLE_NUM_R {
        PRE_IDLE_NUM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pre_idle_num(&mut self) -> PRE_IDLE_NUM_W {
        PRE_IDLE_NUM_W { w: self }
    }
}
