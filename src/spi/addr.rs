#[doc = "Reader of register ADDR"]
pub type R = crate::R<u32, super::ADDR>;
#[doc = "Writer for register ADDR"]
pub type W = crate::W<u32, super::ADDR>;
#[doc = "Register ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USR_ADDR_VALUE`"]
pub type USR_ADDR_VALUE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `USR_ADDR_VALUE`"]
pub struct USR_ADDR_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_ADDR_VALUE_W<'a> {
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
    pub fn usr_addr_value(&self) -> USR_ADDR_VALUE_R {
        USR_ADDR_VALUE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn usr_addr_value(&mut self) -> USR_ADDR_VALUE_W {
        USR_ADDR_VALUE_W { w: self }
    }
}
